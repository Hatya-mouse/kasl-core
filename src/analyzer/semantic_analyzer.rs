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

use knodiq_engine::Type;

use crate::{
    Expression, Function, Program, SemanticError, Statement, SymbolInfo, SymbolKind, semantic_error,
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct SemanticAnalyzer {
    pub symbol_table: HashMap<String, SymbolInfo>,
    pub var_table: HashMap<String, SymbolInfo>,
    pub input_table: HashMap<String, SymbolInfo>,
    pub output_table: HashMap<String, SymbolInfo>,
    pub function_table: HashMap<String, Function>,
    error: SemanticError,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            symbol_table: HashMap::new(),
            var_table: HashMap::new(),
            input_table: HashMap::new(),
            output_table: HashMap::new(),
            function_table: HashMap::new(),
            error: SemanticError::new(),
        }
    }

    pub fn analyze(&mut self, program: &Program) -> Result<(), SemanticError> {
        self.analyze_statements(&program.statements)
    }

    pub fn analyze_statements(&mut self, statements: &Vec<Statement>) -> Result<(), SemanticError> {
        self.function_table
            .extend(crate::builtin_function::built_in_functions());

        for statement in statements {
            match statement {
                Statement::InputDeclaration(input) => {
                    let name = input.name.clone();

                    self.define_input(
                        name.clone(),
                        SymbolInfo {
                            name: name.clone(),
                            kind: SymbolKind::Input,
                            value_type: input.value_type.clone(),
                            value: None,
                        },
                    );
                }

                Statement::OutputDeclaration(output) => {
                    let name = output.name.clone();

                    self.define_output(
                        name.clone(),
                        SymbolInfo {
                            name: name.clone(),
                            kind: SymbolKind::Output,
                            value_type: output.value_type.clone(),
                            value: None,
                        },
                    );
                }

                Statement::VariableDeclaration(var) => {
                    let name = var.name.clone();

                    self.define_symbol(
                        name.clone(),
                        SymbolInfo {
                            name: name.clone(),
                            kind: SymbolKind::Variable,
                            value_type: var.value_type.clone(),
                            value: None,
                        },
                    );
                }

                Statement::Assignment(assignment) => {
                    let target = &assignment.target_name;

                    if !self.symbol_table.contains_key(target) {
                        self.error
                            .errors
                            .push(semantic_error::ErrorVariant::UndefinedSymbol(
                                target.clone(),
                            ));
                    }

                    if let Err(e) = self.analyze_expression(&assignment.value) {
                        self.error.errors.extend(e.errors);
                    }
                }

                Statement::ForLoop(loop_stmt) => {
                    let variable_name = &loop_stmt.variable_name;

                    if self.symbol_table.contains_key(variable_name) {
                        self.error
                            .errors
                            .push(semantic_error::ErrorVariant::SymbolAlreadyDefined(
                                variable_name.clone(),
                            ));
                    } else {
                        self.define_symbol(
                            variable_name.clone(),
                            SymbolInfo {
                                name: variable_name.clone(),
                                kind: SymbolKind::Variable,
                                value_type: Type::Int,
                                value: None,
                            },
                        );
                    }

                    // Check if the body is valid
                    if let Err(e) = self.analyze_statements(&loop_stmt.body) {
                        self.error.errors.extend(e.errors);
                    }
                }
            }
        }

        if self.error.errors.is_empty() {
            Ok(())
        } else {
            Err(self.error.clone())
        }
    }

    pub fn analyze_expression(&mut self, expr: &Expression) -> Result<(), SemanticError> {
        match expr {
            Expression::Literal(_) => Ok(()),
            Expression::Identifier(name) => {
                if !self.symbol_table.contains_key(name) {
                    self.error
                        .errors
                        .push(semantic_error::ErrorVariant::UndefinedSymbol(name.clone()));
                }
                Ok(())
            }
            Expression::FunctionCall { name, arguments } => {
                if !self.function_table.contains_key(name) {
                    self.error
                        .errors
                        .push(semantic_error::ErrorVariant::UndefinedFunction(
                            name.clone(),
                        ));
                }
                for arg in arguments {
                    self.analyze_expression(arg)?;
                }
                Ok(())
            }
            Expression::BinaryOp { op: _, left, right } => {
                self.analyze_expression(left)?;
                self.analyze_expression(right)
            }
        }
    }

    pub fn get_input_table(&self) -> HashMap<String, SymbolInfo> {
        self.input_table.clone()
    }

    pub fn get_output_table(&self) -> HashMap<String, SymbolInfo> {
        self.output_table.clone()
    }

    fn define_symbol(&mut self, name: String, info: SymbolInfo) {
        if self.symbol_table.contains_key(&name) {
            self.error
                .errors
                .push(semantic_error::ErrorVariant::SymbolAlreadyDefined(
                    name.clone(),
                ));
        } else {
            self.symbol_table.insert(name.clone(), info.clone());
            self.var_table.insert(name, info);
        }
    }

    fn define_input(&mut self, name: String, info: SymbolInfo) {
        if self.input_table.contains_key(&name) {
            self.error
                .errors
                .push(semantic_error::ErrorVariant::SymbolAlreadyDefined(
                    name.clone(),
                ));
        } else {
            self.symbol_table.insert(name.clone(), info.clone());
            self.input_table.insert(name, info);
        }
    }

    fn define_output(&mut self, name: String, info: SymbolInfo) {
        if self.output_table.contains_key(&name) {
            self.error
                .errors
                .push(semantic_error::ErrorVariant::SymbolAlreadyDefined(
                    name.clone(),
                ));
        } else {
            self.symbol_table.insert(name.clone(), info.clone());
            self.output_table.insert(name, info);
        }
    }
}
