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
    ConstructorError, ConstructorErrorType, ParserStatementKind, Program, Range,
    SymbolPathComponent, SymbolTable,
    resolution::{
        dependency_analysis::{build_graph, sort_graph},
        symbol_locator::ProgramLocator,
    },
};

pub fn resolve_types(
    program: &mut Program,
    symbol_table: &SymbolTable,
) -> Result<(), Vec<ConstructorError>> {
    // Build the type dependency graph
    let graph = build_graph(symbol_table);

    // Then sort symbols based on the dependency graph
    let sorted_list = match sort_graph(&graph) {
        Ok(sorted_list) => sorted_list,
        Err(causative_symbols) => {
            let errors = causative_symbols
                .into_iter()
                .map(|symbol| {
                    // Get the symbol declaration statement
                    let symbol_decl_statement = symbol_table.get_statement_by_path(&symbol);
                    // And get the range in which the statement is declared
                    let symbol_decl_position = symbol_decl_statement.map(|stmt| stmt.range);
                    ConstructorError {
                        error_type: ConstructorErrorType::DependencyCycle(symbol),
                        position: symbol_decl_position.unwrap_or(Range::zero()),
                    }
                })
                .collect();
            return Err(errors);
        }
    };

    // Resolve types for each symbol in the sorted order
    let mut errors = Vec::new();

    for symbol_path in sorted_list {
        // Get the symbol declaration statement
        let symbol_decl_statement = match symbol_table.get_statement_by_path(symbol) {
            Some(stmt) => stmt,
            None => {
                continue;
            }
        };

        // Check if the symbol has a type annotation
        // If not, infer the type
        match symbol_decl_statement.kind {
            ParserStatementKind::Var {
                required_by,
                name,
                value_type,
                def_val,
            } => {
                if let Some(value_type_path) = value_type {
                    let value_type = program.get_type_def_by_path(value_type_path);
                } else {
                }
            }
            _ => (),
        }
    }

    Ok(())
}
