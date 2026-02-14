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
    ParserOperatorType, ParserStatement, ParserStatementKind, SymbolTable,
    error::{ErrorCollector, Ph},
};

/// Check for errors in the given symbol table.
pub fn validate(ec: &mut ErrorCollector, symbol_table: &SymbolTable) {
    let infix_funcs = &symbol_table
        .infix_funcs
        .values()
        .flatten()
        .collect::<Vec<&&ParserStatement>>();

    for stmt in infix_funcs {
        match &stmt.kind {
            ParserStatementKind::OperatorFunc {
                op_type,
                symbol: _,
                params,
                return_type: _,
                body: _,
            } => match op_type {
                ParserOperatorType::Infix => {
                    if params.len() != 2 {
                        ec.invalid_param_numbers_for_infix(
                            stmt.range,
                            Ph::TopLevelCollection,
                            params.len(),
                        );
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }

    let prefix_funcs = &symbol_table
        .prefix_funcs
        .values()
        .flatten()
        .collect::<Vec<&&ParserStatement>>();

    for stmt in prefix_funcs {
        match &stmt.kind {
            ParserStatementKind::OperatorFunc {
                op_type,
                symbol: _,
                params,
                return_type: _,
                body: _,
            } => match op_type {
                ParserOperatorType::Prefix => {
                    if params.len() != 1 {
                        ec.invalid_param_numbers_for_prefix(
                            stmt.range,
                            Ph::TopLevelCollection,
                            params.len(),
                        );
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
}
