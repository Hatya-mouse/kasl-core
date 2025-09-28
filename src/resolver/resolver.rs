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
    FuncParam, Function, InputVar, OutputVar, ParserStatement, Program, ResolverErrorType,
    StateVar, TypeDef, Variable,
    parser_ast::ParserStatementKind,
    resolver_error::ResolverError,
    resolver_error_type::{ScopeType, StatementType},
};

pub struct Resolver {
    statements: Vec<ParserStatement>,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    pub fn resolve(&mut self, statements: Vec<ParserStatement>) -> Result<(), ResolverError> {
        let mut program = Program::new();
        self.statements = statements;

        program.types = Resolver::collect_types(&self.statements);

        Ok(())
    }

    pub fn collect_types(stmts: &Vec<ParserStatement>) -> Vec<TypeDef> {
        let mut types = Vec::new();

        for stmt in stmts {
            match &stmt.kind {
                ParserStatementKind::StructDecl {
                    name,
                    inherits: _,
                    body,
                }
                | ParserStatementKind::ProtocolDecl {
                    name,
                    inherits: _,
                    body,
                } => {
                    let mut type_def = TypeDef::new(name.clone());
                    let child_types = Resolver::collect_types(&body);
                    type_def.types = child_types;
                    types.push(type_def);
                }
                _ => (),
            }
        }

        types
    }

    pub fn collect_symbols(
        program: &mut Program,
        stmts: &Vec<ParserStatement>,
    ) -> Result<(), ResolverError> {
        for stmt in stmts {
            match &stmt.kind {
                ParserStatementKind::Input {
                    name,
                    value_type,
                    def_val: _,
                    attrs: _,
                } => {
                    let resolved_type = match value_type {
                        Some(ty) => Some(program.resolve_type(ty)?),
                        None => None,
                    };
                    program.inputs.push(InputVar {
                        name: name.to_string(),
                        value_type: resolved_type,
                        def_val: None,
                        attrs: Vec::new(),
                    });
                }
                ParserStatementKind::Output { name, value_type } => {
                    let resolved_type = program.resolve_type(value_type)?;
                    program.outputs.push(OutputVar {
                        name: name.to_string(),
                        value_type: resolved_type,
                    })
                }
                ParserStatementKind::State { vars } => {
                    for var in vars {
                        let resolved_type = match var.value_type {
                            Some(ref ty) => Some(program.resolve_type(ty)?),
                            None => None,
                        };
                        program.states.push(StateVar {
                            name: var.name.to_string(),
                            value_type: resolved_type,
                            def_val: None,
                        });
                    }
                }
                ParserStatementKind::FuncDecl {
                    required_by,
                    name,
                    params,
                    return_type,
                    body: _,
                } => {
                    if required_by.is_some() {
                        return Err(ResolverError {
                            offset: stmt.start,
                            error_type: ResolverErrorType::InvalidRequiredBy,
                        });
                    }

                    let resolved_return_type = match return_type {
                        Some(ty) => Some(program.resolve_type(ty)?),
                        None => None,
                    };

                    let params_result: Result<Vec<_>, _> = params
                        .iter()
                        .map(|param| {
                            Ok(FuncParam {
                                label: param.label.clone(),
                                name: param.name.clone(),
                                value_type: match param.value_type {
                                    Some(ref ty) => Some(program.resolve_type(ty)?),
                                    None => None,
                                },
                                def_val: None,
                            })
                        })
                        .collect();

                    program.funcs.push(Function {
                        name: name.to_string(),
                        params: params_result?,
                        return_type: resolved_return_type,
                        body: Vec::new(),
                        required_by: None,
                    })
                }
                ParserStatementKind::Var {
                    required_by: _,
                    name: _,
                    value_type: _,
                    def_val: _,
                } => {
                    return Err(ResolverError {
                        offset: stmt.start,
                        error_type: ResolverErrorType::Invalid {
                            scope: ScopeType::TopLevel,
                            cause: StatementType::Var,
                        },
                    });
                }
                _ => (),
            }
        }

        Ok(())
    }

    pub fn collect_members(
        program: &Program,
        stmts: &Vec<ParserStatement>,
        type_def: &mut TypeDef,
    ) -> Result<(), ResolverError> {
        for stmt in stmts {
            match &stmt.kind {
                ParserStatementKind::Var {
                    required_by: _,
                    name,
                    value_type,
                    def_val: _,
                } => type_def.vars.push(Variable {
                    name: name.clone(),
                    value_type: match value_type {
                        Some(ty) => Some(program.resolve_type(ty)?),
                        None => None,
                    },
                    def_val: None,
                }),
                ParserStatementKind::FuncDecl {
                    required_by,
                    name,
                    params,
                    return_type,
                    body: _,
                } => {
                    let resolved_required_by = match required_by {
                        Some(ty) => Some(program.resolve_type(ty)?),
                        None => None,
                    };
                    let resolved_return_type = match return_type {
                        Some(ty) => Some(program.resolve_type(ty)?),
                        None => None,
                    };

                    let params_result: Result<Vec<_>, _> = params
                        .iter()
                        .map(|param| {
                            Ok(FuncParam {
                                label: param.label.clone(),
                                name: param.name.clone(),
                                value_type: match param.value_type {
                                    Some(ref ty) => Some(program.resolve_type(ty)?),
                                    None => None,
                                },
                                def_val: None,
                            })
                        })
                        .collect();

                    type_def.funcs.push(Function {
                        name: name.to_string(),
                        params: params_result?,
                        return_type: resolved_return_type,
                        body: Vec::new(),
                        required_by: resolved_required_by,
                    });
                }
                ParserStatementKind::StructDecl {
                    name,
                    inherits,
                    body,
                } => {
                    let child_type_def = type_def.fine_type_def_mut(name);
                    match child_type_def {
                        Some(child_type_def) => {
                            let child_type_inherits: Result<Vec<_>, _> =
                                inherits.iter().map(|ty| program.resolve_type(ty)).collect();
                            child_type_def.inherits = child_type_inherits?;
                            Self::collect_members(program, body, child_type_def)?;
                        }
                        None => {
                            return Err(ResolverError {
                                error_type: ResolverErrorType::TypeNotFound(name.to_string()),
                                offset: 0,
                            });
                        }
                    }
                }
                _ => (),
            }
        }

        Ok(())
    }
}
