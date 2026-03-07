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
    Function, MAIN_FUNCTION_NAME, PrimitiveType, Program, compilation::InputProvider,
    data::VariableID, symbol_path,
};
use cranelift::prelude::*;
use std::collections::HashMap;

pub struct Translator<'a> {
    pub builder: FunctionBuilder<'a>,
    pub program: &'a Program,
    pub primitive_types: HashMap<VariableID, PrimitiveType>,
    pub input_provider: dyn InputProvider,
}

impl<'a> Translator<'a> {
    /// Translates the given AST into Cranelift IR.
    pub fn translate(&mut self) {
        let main_func = self.get_main_func();
        self.translate_block(&main_func.body);
    }

    /// Returns a reference to the main function of the given program.
    /// Assumes that it is safe to unwrap the ID and function from the program.
    fn get_main_func(&self) -> &'a Function {
        let main_func_path = symbol_path![MAIN_FUNCTION_NAME.to_string()];
        let main_func_id = self
            .program
            .get_id_by_path(&main_func_path)
            .unwrap()
            .first()
            .unwrap();
        self.program.get_func(main_func_id).unwrap()
    }
}
