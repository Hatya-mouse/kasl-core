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

use crate::{Expression, Scope, Statement, TypeDef};

#[derive(Debug, PartialEq, Clone)]
pub struct Function<'a> {
    pub name: String,
    pub params: Vec<FuncParam<'a>>,
    pub return_type: Option<&'a TypeDef<'a>>,
    pub body: Vec<Statement<'a>>,
    pub required_by: Option<&'a TypeDef<'a>>,
}

impl<'a> Scope<'a> for Function<'a> {
    fn get_func_mut(&mut self, _name: &str) -> Option<&mut Function<'a>> {
        None
    }

    fn get_type_def_mut(&mut self, _name: &str) -> Option<&mut TypeDef<'a>> {
        None
    }

    fn get_state_mut(&mut self, _name: &str) -> Option<&mut super::StateVar<'a>> {
        None
    }

    fn get_input_mut(&mut self, _name: &str) -> Option<&mut super::InputVar<'a>> {
        None
    }

    fn get_output_mut(&mut self, _name: &str) -> Option<&mut super::OutputVar<'a>> {
        None
    }

    fn get_var_mut(&mut self, _name: &str) -> Option<&mut super::Variable<'a>> {
        None
    }

    fn get_operator_mut(&mut self, _name: &str) -> Option<&mut super::Operator<'a>> {
        None
    }

    fn get_func_param_mut(&mut self, name: &str) -> Option<&mut FuncParam<'a>> {
        self.params.iter_mut().find(|p| p.name == name)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FuncParam<'a> {
    pub label: Option<String>,
    pub name: String,
    pub value_type: Option<&'a TypeDef<'a>>,
    pub def_val: Option<Box<Expression<'a>>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncCallArg<'a> {
    pub label: String,
    pub value: Expression<'a>,
}
