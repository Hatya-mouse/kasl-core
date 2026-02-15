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
    ExprToken, ParserOperatorType, ParserStatementKind, ParserSymbolPath, Program, Range,
    SymbolPath, SymbolTable,
    ast::tree_items::variables::VariableTrait,
    error::{ErrorCollector, Phase},
    resolution::{
        dependency_analysis::{build_graph, sort_graph},
        expr_inference::ExprTreeBuilder,
        resolvers::{
            infix_operator_resolver::resolve_infix_func,
            prefix_operator_resolver::resolve_prefix_operator,
        },
    },
    symbol_table::symbol_table::StatementLookup,
};

/// Infer the types of symbols (input, output, state, var, and function parameters) in the program.
pub fn resolve_types(ec: &mut ErrorCollector, program: &mut Program, symbol_table: &SymbolTable) {
    // Build the type dependency graph
    let graph = match build_graph(ec, symbol_table) {
        Some(graph) => graph,
        None => return,
    };

    // Then sort symbols based on the dependency graph
    let sorted_list = match sort_graph(&graph) {
        Ok(sorted_list) => sorted_list,
        Err(causative_symbols) => {
            for symbol_path in causative_symbols {
                if let Some(current_stmt) = symbol_table.get_statement_by_path(&symbol_path) {
                    match current_stmt {
                        StatementLookup::Single(stmt) => {
                            // And get the range in which the statement is declared
                            ec.dep_cycle(
                                stmt.range,
                                Phase::TypeResolution,
                                &symbol_path.to_string(),
                            );
                        }
                        StatementLookup::Multiple(stmts) => {
                            // Iterate over each statement and push an error for each one
                            for stmt in stmts {
                                ec.dep_cycle(
                                    stmt.range,
                                    Phase::TypeResolution,
                                    &symbol_path.to_string(),
                                );
                            }
                        }
                    }
                } else {
                    ec.dep_cycle(
                        Range::zero(),
                        Phase::TypeResolution,
                        &symbol_path.to_string(),
                    );
                }
            }
            return;
        }
    };

    let mut statements = Vec::new();
    for symbol_path in sorted_list {
        // Get the symbol declaration statement
        if let Some(current_stmt) = symbol_table.get_statement_by_path(&symbol_path) {
            match current_stmt {
                StatementLookup::Single(stmt) => {
                    statements.push((symbol_path, stmt));
                }
                StatementLookup::Multiple(stmts) => {
                    for stmt in stmts {
                        statements.push((symbol_path, stmt));
                    }
                }
            }
        } else {
            ec.comp_bug(Range::zero(), Phase::TypeResolution, "");
        }
    }

    // Infer the type of each symbol in the sorted order
    for (symbol_path, current_stmt) in statements {
        // Check if the symbol has already got a type annotation
        // If not, infer the type
        match &current_stmt.kind {
            ParserStatementKind::Input {
                name,
                value_type,
                def_val,
                attrs: _,
            } => infer_type_and_write(
                ec,
                program,
                symbol_table,
                symbol_path,
                current_stmt.range,
                |program| program.get_input_mut(name),
                Some(def_val),
                value_type.as_ref(),
            ),

            ParserStatementKind::Output {
                name,
                value_type,
                def_val,
            } => infer_type_and_write(
                ec,
                program,
                symbol_table,
                symbol_path,
                current_stmt.range,
                |program| program.get_output_mut(name),
                Some(def_val),
                value_type.as_ref(),
            ),

            ParserStatementKind::State { vars } => {
                for var in vars {
                    infer_type_and_write(
                        ec,
                        program,
                        symbol_table,
                        symbol_path,
                        current_stmt.range,
                        |program| program.get_state_mut(&var.name),
                        Some(&var.def_val),
                        var.value_type.as_ref(),
                    );
                }
            }

            ParserStatementKind::Var {
                required_by: _,
                name: _,
                value_type,
                def_val,
            } => infer_type_and_write(
                ec,
                program,
                symbol_table,
                symbol_path,
                current_stmt.range,
                |program| program.get_var_by_path_mut(symbol_path),
                Some(def_val),
                value_type.as_ref(),
            ),

            ParserStatementKind::FuncDecl {
                required_by: _,
                name,
                params,
                return_type,
                body: _,
            } => {
                // If the function has a return type, resolve it
                if let Some(return_type) = return_type {
                    if let Some(return_type_path) =
                        program.resolve_type_def_parser_path(&return_type)
                    {
                        match program.get_func_mut(name) {
                            Some(func) => func.return_type = Some(return_type_path),
                            None => {
                                ec.func_not_found(current_stmt.range, Phase::TypeResolution, name)
                            }
                        }
                    } else {
                        ec.type_not_found(
                            current_stmt.range,
                            Phase::TypeResolution,
                            &return_type.to_string(),
                        );
                    }
                }

                for param in params {
                    infer_type_and_write(
                        ec,
                        program,
                        symbol_table,
                        symbol_path,
                        current_stmt.range,
                        |program| program.get_func_param_by_path_mut(symbol_path, &param.name),
                        param.def_val.as_ref(),
                        param.value_type.as_ref(),
                    );
                }
            }

            ParserStatementKind::OperatorFunc {
                op_type,
                symbol,
                params,
                return_type,
                body: _,
            } => match op_type {
                ParserOperatorType::Infix => resolve_infix_func(
                    ec,
                    program,
                    symbol_table,
                    symbol,
                    params,
                    return_type,
                    current_stmt.range,
                ),
                ParserOperatorType::Prefix => resolve_prefix_operator(
                    ec,
                    program,
                    symbol_table,
                    symbol,
                    params,
                    return_type,
                    current_stmt.range,
                ),
            },

            _ => (),
        }
    }
}

/// Infer the type of the variable and write it to the Program.
///
/// # Arguments
/// - `program`: Mutable reference to the Program.
/// - `symbol_path`: SymbolPath of the variable to infer type for.
/// - `decl_range`: Range of the declaration statement.
/// - `get_target`: Function to get the target variable.
fn infer_type_and_write<T, F>(
    ec: &mut ErrorCollector,
    program: &mut Program,
    symbol_table: &SymbolTable,
    symbol_path: &SymbolPath,
    decl_range: Range,
    get_target: F,
    default_value: Option<&Vec<ExprToken>>,
    type_annotation: Option<&ParserSymbolPath>,
) where
    F: for<'a> Fn(&'a mut Program) -> Option<&'a mut T>,
    T: VariableTrait + Sized + std::fmt::Debug,
{
    if let Some(default_value) = default_value {
        let parsed_expr =
            match program.build_expr_tree_from_raw_tokens(ec, default_value, symbol_table) {
                Some(expr) => expr,
                None => return,
            };
        let def_val_type = match parsed_expr.get_type(ec, program, decl_range) {
            Some(t) => t,
            None => return,
        };

        if let Some(type_annotation) = type_annotation {
            if let Some(annotation_type) = program.resolve_type_def_parser_path(type_annotation) {
                // Check if the type annotation matches the inferred type
                if annotation_type == def_val_type {
                    let target_variable = match get_target(program) {
                        Some(target) => target,
                        None => {
                            ec.comp_bug(
                                decl_range,
                                Phase::TypeResolution,
                                "Symbol path generated by build_graph() is invalid.",
                            );
                            return;
                        }
                    };
                    target_variable.set_default_value(Some(parsed_expr));
                    target_variable.set_value_type(Some(annotation_type));
                } else {
                    // If the type annotation doesn't match the inferred type, throw an error
                    ec.type_mismatch(
                        decl_range,
                        Phase::TypeResolution,
                        &annotation_type.to_string(),
                        &def_val_type.to_string(),
                    );
                    return;
                }
            } else {
                // If the type annotation is not found, throw an error
                ec.type_not_found(
                    decl_range,
                    Phase::TypeResolution,
                    &type_annotation.to_string(),
                );
                return;
            }
        } else {
            // If the symbol doesn't have a type annotation, use the inferred one
            let target_variable = match get_target(program) {
                Some(target) => target,
                None => {
                    ec.comp_bug(
                        decl_range,
                        Phase::TypeResolution,
                        "Symbol path generated by build_graph() is invalid.",
                    );
                    return;
                }
            };
            target_variable.set_value_type(Some(def_val_type));
        }
    } else {
        if let Some(type_annotation) = type_annotation {
            if let Some(type_symbol_path) = program.resolve_type_def_parser_path(type_annotation) {
                // If the symbol has a type annotation, use it
                let target_variable = match get_target(program) {
                    Some(target) => target,
                    None => {
                        ec.comp_bug(
                            decl_range,
                            Phase::TypeResolution,
                            "Symbol path generated by build_graph() is invalid.",
                        );
                        return;
                    }
                };
                target_variable.set_value_type(Some(type_symbol_path));
            } else {
                // If the type annotation is not found, push an error
                ec.type_not_found(
                    decl_range,
                    Phase::TypeResolution,
                    &type_annotation.to_string(),
                );
                return;
            }
        } else {
            // If the symbol doesn't have the both type annotation and inferred type, push an error
            ec.missing_type_annotation(decl_range, Phase::TypeResolution, &symbol_path.to_string());
            return;
        }
    }
}
