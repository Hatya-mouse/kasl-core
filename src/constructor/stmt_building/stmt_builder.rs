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
    FuncCallArg, ParserBodyStmt, ParserBodyStmtKind, Program, Statement, SymbolTable,
    error::{ErrorCollector, Phase},
    resolution::expr_inference::ExprTreeBuilder,
};

pub fn build_statements(
    ec: &mut ErrorCollector,
    program: &Program,
    symbol_table: &SymbolTable,
    original_stmts: &[ParserBodyStmt],
) -> Vec<Statement> {
    let mut parsed_stmts = Vec::new();

    for stmt in original_stmts {
        match &stmt.kind {
            ParserBodyStmtKind::Assign { target, value } => {
                let parsed_target = match symbol_table.resolve_path(target) {
                    Some(parsed_target) => parsed_target,
                    None => {
                        ec.var_not_found(stmt.range, Phase::StatementBuilding, &target.to_string());
                        continue;
                    }
                };

                let parsed_value =
                    match program.build_expr_tree_from_raw_tokens(ec, value, symbol_table) {
                        Some(parsed_value) => parsed_value,
                        None => {
                            // Error should have been reported the function so we don't need to report it here
                            continue;
                        }
                    };

                // Create an Assign statement
                let assign_stmt = Statement::Assign {
                    target: parsed_target,
                    value: parsed_value,
                };
                parsed_stmts.push(assign_stmt);
            }

            ParserBodyStmtKind::Block { statements } => {
                // Build statements within the block
                let block_body = build_statements(ec, program, symbol_table, statements);
                // Create a Block statement
                let block_stmt = Statement::Block { body: block_body };
                parsed_stmts.push(block_stmt);
            }

            ParserBodyStmtKind::FuncCall { path, args } => {
                let func_path = match symbol_table.resolve_path(path) {
                    Some(parsed_target) => parsed_target,
                    None => {
                        ec.func_not_found(stmt.range, Phase::StatementBuilding, &path.to_string());
                        continue;
                    }
                };
                let target_func = match program.get_func_by_path(&func_path) {
                    Some(func) => func,
                    None => {
                        ec.func_not_found(
                            stmt.range,
                            Phase::StatementBuilding,
                            &func_path.to_string(),
                        );
                        continue;
                    }
                };

                // Check if the number of arguments is within the valid range
                let minimum_num = target_func.min_num_of_params();
                let maximum_num = target_func.max_num_of_params();
                let actual_num = args.len();

                if actual_num < minimum_num {
                    ec.not_enough_params(
                        stmt.range,
                        Phase::StatementBuilding,
                        &func_path.to_string(),
                        minimum_num,
                        actual_num,
                    );
                    continue;
                }

                if actual_num > maximum_num {
                    ec.too_many_params(
                        stmt.range,
                        Phase::StatementBuilding,
                        &func_path.to_string(),
                        maximum_num,
                        actual_num,
                    );
                    continue;
                }

                // Get the name of the parameters
                let mut parsed_args = Vec::new();
                let i = 0;
                for arg in args {
                    let parsed_value =
                        match program.build_expr_tree_from_raw_tokens(ec, &arg.value, symbol_table)
                        {
                            Some(parsed_value) => parsed_value,
                            None => continue,
                        };
                    let arg_name = match &arg.label {
                        Some(label) => match target_func.get_param_name_by_label(label) {
                            Some(name) => name,
                            None => {
                                ec.param_not_found(
                                    stmt.range,
                                    Phase::StatementBuilding,
                                    &func_path.to_string(),
                                    label,
                                );
                                continue;
                            }
                        },
                        None => match target_func.get_param_name_by_index(i) {
                            Some(name) => name,
                            None => continue,
                        },
                    };

                    let parsed_arg = FuncCallArg {
                        name: arg_name,
                        value: parsed_value,
                    };
                    parsed_args.push(parsed_arg);
                }

                // Create a FuncCall statement
                let func_call_stmt = Statement::FuncCall {
                    path: func_path,
                    args: parsed_args,
                };
                parsed_stmts.push(func_call_stmt);
            }
            _ => (),
        }
    }

    parsed_stmts
}
