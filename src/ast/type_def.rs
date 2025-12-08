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

use crate::{FuncParam, Function, Operator, Scope, Statement, SymbolPath, Variable};

#[derive(Debug, PartialEq, Clone)]
pub struct TypeDef {
    pub name: String,
    pub inherits: Vec<SymbolPath>,
    pub vars: Vec<Variable>,
    pub inits: Vec<Initializer>,
    pub funcs: Vec<Function>,
    pub types: Vec<TypeDef>,
    pub operators: Vec<Operator>,
}

impl TypeDef {
    pub fn new(name: String) -> Self {
        TypeDef {
            name,
            inherits: Vec::new(),
            vars: Vec::new(),
            inits: Vec::new(),
            funcs: Vec::new(),
            types: Vec::new(),
            operators: Vec::new(),
        }
    }

    pub fn get_type_def_mut(&mut self, name: &str) -> Option<&mut TypeDef> {
        self.types.iter_mut().find(|s| s.name == name)
    }
}

impl Scope for TypeDef {
    fn get_func_mut(&mut self, name: &str) -> Option<&mut Function> {
        self.funcs.iter_mut().find(|f| f.name == name)
    }

    fn get_type_def_mut(&mut self, name: &str) -> Option<&mut TypeDef> {
        self.types.iter_mut().find(|s| s.name == name)
    }

    fn get_state_mut(&mut self, _name: &str) -> Option<&mut super::StateVar> {
        None
    }

    fn get_input_mut(&mut self, _name: &str) -> Option<&mut super::InputVar> {
        None
    }

    fn get_output_mut(&mut self, _name: &str) -> Option<&mut super::OutputVar> {
        None
    }

    fn get_var_mut(&mut self, name: &str) -> Option<&mut Variable> {
        self.vars.iter_mut().find(|v| v.name == name)
    }

    fn get_operator_mut(&mut self, name: &str) -> Option<&mut Operator> {
        self.operators.iter_mut().find(|o| o.symbol == name)
    }

    fn get_func_param_mut(&mut self, _name: &str) -> Option<&mut FuncParam> {
        None
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Initializer {
    pub literal_bind: Option<LiteralBind>,
    pub params: Vec<FuncParam>,
    pub body: Vec<Statement>,
    pub required_by: Option<SymbolPath>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralBind {
    IntLiteral,
    FloatLiteral,
    BoolLiteral,
}
