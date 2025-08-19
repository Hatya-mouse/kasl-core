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

use crate::{
    ParserStatement, ProtocolType, StructType, parser_ast::ParserStatementKind,
    symbol_table::SymbolTable,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ResolverError {
    offset: usize,
    message: String,
}

pub struct Resolver {
    program: Vec<ParserStatement>,
    root_symbol_table: SymbolTable,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            program: Vec::new(),
            root_symbol_table: SymbolTable::new(),
        }
    }

    pub fn resolve(&mut self, program: Vec<ParserStatement>) -> Result<(), ResolverError> {
        self.program = program;
        self.register_types();
        Ok(())
    }

    pub fn register_types(&mut self) {
        for stmt in &self.program {
            match &stmt.kind {
                ParserStatementKind::StructDecl {
                    name,
                    inherits: _,
                    body: _,
                } => {
                    self.root_symbol_table.add_struct(StructType {
                        name: name.clone(),
                        inherits: Vec::new(),
                        vars: Vec::new(),
                        inits: Vec::new(),
                        funcs: Vec::new(),
                    });
                }
                ParserStatementKind::ProtocolDecl {
                    name,
                    inherits: _,
                    requires: _,
                } => {
                    self.root_symbol_table.add_protocol(ProtocolType {
                        name: name.clone(),
                        inherits: Vec::new(),
                        vars: Vec::new(),
                        funcs: Vec::new(),
                    });
                }
                _ => {}
            }
        }
    }
}
