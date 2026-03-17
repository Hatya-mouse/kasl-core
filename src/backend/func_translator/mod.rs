//
// © 2025-2026 Shuntaro Kasatani
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

mod block_translator;
mod expr_translators;
mod io_blueprint;
mod stmt_translators;
mod type_converter;

use cranelift_codegen::ir;
pub use type_converter::TypeConverter;

use crate::{
    FunctionID, VariableID,
    builtin::BuiltinRegistry,
    compilation_data::ProgramContext,
    scope_manager::{IOBlueprint, VariableKind},
};
use cranelift::prelude::{FunctionBuilder, Variable};
use cranelift_jit::JITModule;
use std::collections::HashMap;

pub struct FuncTranslator<'a> {
    pub builder: FunctionBuilder<'a>,
    type_converter: TypeConverter,

    prog_ctx: &'a ProgramContext,
    builtin_registry: &'a BuiltinRegistry,
    variables: HashMap<VariableID, Variable>,
}

impl<'a> FuncTranslator<'a> {
    pub fn new(
        builder: FunctionBuilder<'a>,
        module: &'a JITModule,
        prog_ctx: &'a ProgramContext,
        builtin_registry: &'a BuiltinRegistry,
    ) -> Self {
        let type_converter = TypeConverter::new(module);

        Self {
            builder,
            type_converter,
            prog_ctx,
            builtin_registry,
            variables: HashMap::new(),
        }
    }

    pub fn translate(
        &mut self,
        entry_point: &FunctionID,
        blueprint: &IOBlueprint,
        entry_block: ir::Block,
        return_block: ir::Block,
    ) {
        // Get the pointer to the pointer array
        let block_params = self.builder.block_params(entry_block);
        let input_ptr_ptr = block_params[0];
        let output_ptr_ptr = block_params[1];
        let state_ptr_ptr = block_params[2];

        // Get the input and state variables from the blueprint
        self.load_blueprint_access(input_ptr_ptr, state_ptr_ptr, blueprint);

        // Declare the output variables
        let root_namespace_id = self.prog_ctx.namespace_registry.get_root_namespace_id();
        let global_scope = self
            .prog_ctx
            .scope_registry
            .get_global_scope(&root_namespace_id);
        for var_id in &global_scope.variables {
            // Get the variable
            let Some(scope_var) = self.prog_ctx.scope_registry.get_var(var_id) else {
                continue;
            };

            if matches!(&scope_var.var_kind, VariableKind::Output) {
                let output_var = self.declare_var(*var_id, &scope_var.value_type);
                // Output variables must have a default value
                let def_val = self.translate_expr(scope_var.def_val.as_ref().unwrap());
                self.builder.def_var(output_var, def_val);
            }
        }

        // Get the entry point function node
        let Some(func_block) = self
            .prog_ctx
            .func_ctx
            .get_func(entry_point)
            .map(|func| &func.block)
        else {
            return;
        };

        self.translate_block(func_block, return_block);

        // Push the values in the states and outputs to the original pointer
        self.store_blueprint(output_ptr_ptr, state_ptr_ptr, blueprint);
    }
}
