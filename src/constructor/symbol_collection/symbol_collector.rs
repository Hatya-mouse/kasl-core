//
// Copyright 2025-2026 Shuntaro Kasatani
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
    Function, InputVar, OutputVar, ParserStatementKind, Program, StateVar, SymbolTable,
    error::{ErrorCollector, Ph},
    member_collection::collectors::construct_func_params,
};

// Collect all symbols from top-level and add them to the symbol table.
pub fn collect_top_level_symbols(
    ec: &mut ErrorCollector,
    program: &mut Program,
    symbol_table: &SymbolTable,
) {
    for stmt in &symbol_table.inputs {
        match &stmt.1.kind {
            ParserStatementKind::Input {
                name,
                value_type: _,
                def_val: _,
                attrs: _,
            } => {
                let input = InputVar {
                    name: name.to_string(),
                    value_type: None,
                    def_val: None,
                    attrs: Vec::new(),
                };
                program.register_input(input);
            }

            _ => (),
        }
    }

    for stmt in &symbol_table.outputs {
        match &stmt.1.kind {
            ParserStatementKind::Output {
                name,
                value_type: _,
                def_val: _,
            } => {
                let output = OutputVar {
                    name: name.to_string(),
                    value_type: None,
                    def_val: None,
                };
                program.register_output(output);
            }

            _ => (),
        }
    }

    for stmt in &symbol_table.states {
        match &stmt.1.kind {
            ParserStatementKind::State { vars } => {
                for var in vars {
                    let state = StateVar {
                        name: var.name.to_string(),
                        value_type: None,
                        def_val: None,
                    };
                    program.register_state(state);
                }
            }

            _ => (),
        }
    }

    for stmt in &symbol_table.funcs {
        match &stmt.1.kind {
            ParserStatementKind::FuncDecl {
                required_by,
                name,
                params,
                return_type: _,
                body: _,
            } => {
                if required_by.is_some() {
                    ec.req_by_outside_type(stmt.1.range, Ph::TopLevelCollection);
                }

                let func_params = construct_func_params(params);
                let function = Function {
                    name: name.to_string(),
                    params: func_params,
                    return_type: None,
                    body: Vec::new(),
                    required_by: None,
                };
                program.register_func(function);
            }

            _ => (),
        }
    }

    for stmt in &symbol_table.infix_defines {
        match &stmt.1.kind {
            ParserStatementKind::InfixDefine {
                symbol,
                infix_properties,
            } => program.register_infix_operator(symbol.to_string(), infix_properties.clone()),
            _ => (),
        }
    }

    for stmt in &symbol_table.prefix_defines {
        match &stmt.1.kind {
            ParserStatementKind::PrefixDefine { symbol } => {
                program.register_prefix_operator(symbol.to_string())
            }
            _ => (),
        }
    }
}
