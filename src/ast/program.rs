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

use crate::{Function, InputVar, OutputVar, Scope, StateVar, TypeDef};

pub struct Program<'a> {
    pub main_func: Option<Function<'a>>,
    pub funcs: Vec<Function<'a>>,
    pub types: Vec<TypeDef<'a>>,
    pub states: Vec<StateVar<'a>>,
    pub inputs: Vec<InputVar<'a>>,
    pub outputs: Vec<OutputVar<'a>>,
}

impl<'a> Program<'a> {
    pub fn new() -> Self {
        Self {
            main_func: None,
            funcs: Vec::new(),
            types: Vec::new(),
            states: Vec::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    /// Get a mutable reference to a Function by name.
    pub fn get_func_mut(&mut self, name: &str) -> Option<&mut Function<'a>> {
        self.funcs.iter_mut().find(|f| f.name == name)
    }

    /// Get a mutable reference to a TypeDef by name.
    pub fn get_type_def_mut(&mut self, name: &str) -> Option<&mut TypeDef<'a>> {
        self.types.iter_mut().find(|s| s.name == name)
    }

    /// Get a mutable reference to a StateVar by name.
    pub fn get_state_mut(&mut self, name: &str) -> Option<&mut StateVar<'a>> {
        self.states.iter_mut().find(|s| s.name == name)
    }

    /// Get a mutable reference to an InputVar by name.
    pub fn get_input_mut(&mut self, name: &str) -> Option<&mut InputVar<'a>> {
        self.inputs.iter_mut().find(|s| s.name == name)
    }

    /// Get a mutable reference to an OutputVar by name.
    pub fn get_output_mut(&mut self, name: &str) -> Option<&mut OutputVar<'a>> {
        self.outputs.iter_mut().find(|s| s.name == name)
    }
}

impl<'a> Scope<'a> for Program<'a> {
    fn get_func_mut(&mut self, name: &str) -> Option<&mut Function<'a>> {
        self.funcs.iter_mut().find(|f| f.name == name)
    }

    fn get_type_def_mut(&mut self, name: &str) -> Option<&mut TypeDef<'a>> {
        self.types.iter_mut().find(|t| t.name == name)
    }

    fn get_state_mut(&mut self, name: &str) -> Option<&mut StateVar<'a>> {
        self.states.iter_mut().find(|s| s.name == name)
    }

    fn get_input_mut(&mut self, name: &str) -> Option<&mut InputVar<'a>> {
        self.inputs.iter_mut().find(|i| i.name == name)
    }

    fn get_output_mut(&mut self, name: &str) -> Option<&mut OutputVar<'a>> {
        self.outputs.iter_mut().find(|o| o.name == name)
    }

    fn get_var_mut(&mut self, _name: &str) -> Option<&mut super::Variable<'a>> {
        None
    }

    fn get_operator_mut(&mut self, _name: &str) -> Option<&mut super::Operator<'a>> {
        None
    }

    fn get_func_param_mut(&mut self, _name: &str) -> Option<&mut super::FuncParam<'a>> {
        None
    }
}
