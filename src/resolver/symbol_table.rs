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

use crate::{Function, ProtocolType, StructType, Variable};

#[derive(Debug, PartialEq, Clone)]
pub struct SymbolTable {
    pub variables: Vec<Variable>,
    pub functions: Vec<Function>,
    pub structs: Vec<StructType>,
    pub protocols: Vec<ProtocolType>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
            functions: Vec::new(),
            structs: Vec::new(),
            protocols: Vec::new(),
        }
    }

    pub fn add_var(&mut self, var: Variable) {
        self.variables.push(var);
    }

    pub fn add_func(&mut self, func: Function) {
        self.functions.push(func);
    }

    pub fn add_struct(&mut self, struct_type: StructType) {
        self.structs.push(struct_type);
    }

    pub fn add_protocol(&mut self, protocol_type: ProtocolType) {
        self.protocols.push(protocol_type);
    }
}
