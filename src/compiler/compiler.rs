//
// Copyright 2025 Shuntaro Kasatani
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

use crate::{Expression, Program, SemanticAnalyzer, Statement};
use cranelift_codegen::{Context, ir::AbiParam};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::Module;

pub struct Compiler {
    ctx: Context,
    module: JITModule,
}

impl Compiler {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let builder = JITBuilder::new(cranelift_module::default_libcall_names())?;
        let module = JITModule::new(builder);

        Ok(Compiler {
            ctx: module.make_context(),
            module,
        })
    }

    pub fn compile(&mut self, program: &Program) -> Result<(), Box<dyn std::error::Error>> {
        let float = self
            .module
            .target_config()
            .pointer_type()
            .double_width()
            .ok_or("Target configuration does not support double width")?;
        let ndarray = self.module.target_config().pointer_type();

        let mut semantic_analyzer = SemanticAnalyzer::new();
        semantic_analyzer.analyze(program)?;
        let inputs = semantic_analyzer.get_input_table();
        let outputs = semantic_analyzer.get_output_table();

        for _ in inputs {
            self.ctx.func.signature.params.push(AbiParam::new(ndarray));
        }

        for _ in outputs {
            self.ctx.func.signature.returns.push(AbiParam::new(ndarray));
        }

        Ok(())
    }

    fn codegen_stmt(&self, statement: &Statement) {
        match statement {
            Statement::InputDeclaration(input_decl) => {}
        }
    }

    fn codegen_expr(&self, expr: &Expression) {
        match expr {}
    }
}
