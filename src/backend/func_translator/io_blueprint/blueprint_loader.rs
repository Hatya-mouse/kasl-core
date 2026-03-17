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

use crate::{
    VariableID,
    backend::func_translator::FuncTranslator,
    scope_manager::{BlueprintItem, IOBlueprint},
    type_registry::ResolvedType,
};
use cranelift::prelude::{InstBuilder, MemFlags};
use cranelift_codegen::ir;

impl FuncTranslator<'_> {
    pub fn load_blueprint_access(
        &mut self,
        input_ptr_ptr: ir::Value,
        state_ptr_ptr: ir::Value,
        blueprint: &IOBlueprint,
    ) {
        // Get the type of a pointer
        let pointer_type = self.type_converter.pointer_type();

        // Loop over the inputs, outputs and states and load them
        // INPUTS
        let mut input_offset: usize = 0;
        for input_item in blueprint.get_inputs() {
            let val = self.load_blueprint_item(
                pointer_type,
                input_ptr_ptr,
                input_item,
                input_offset as i32,
            );
            self.register_translated_var(input_item.id, input_item.value_type, val);
            // Increment the input offset by the size of a pointer
            input_offset += pointer_type.bytes() as usize;
        }

        // STATES
        let mut state_offset: usize = 0;
        for state_item in blueprint.get_states() {
            let val = self.load_blueprint_item(
                pointer_type,
                state_ptr_ptr,
                state_item,
                state_offset as i32,
            );
            self.register_translated_var(state_item.id, state_item.value_type, val);
            // Increment the state offset by the size of a pointer
            state_offset += pointer_type.bytes() as usize;
        }
    }

    fn load_blueprint_item(
        &mut self,
        pointer_type: ir::Type,
        ptr_ptr: ir::Value,
        item: &BlueprintItem,
        offset: i32,
    ) -> ir::Value {
        // Get the pointer to the value by the pointer to the pointers
        let ptr = self
            .builder
            .ins()
            .load(pointer_type, MemFlags::new(), ptr_ptr, offset);

        // Load the value
        self.builder.ins().load(
            self.type_converter.convert(&item.value_type),
            MemFlags::new(),
            ptr,
            0,
        )
    }

    fn register_translated_var(
        &mut self,
        var_id: VariableID,
        var_type: ResolvedType,
        value: ir::Value,
    ) {
        // Declare the variable
        let var = self.declare_var(var_id, &var_type);
        // Register the variable to the variables
        self.variables.insert(var_id, var);
        // Define the variable
        self.builder.def_var(var, value);
    }
}
