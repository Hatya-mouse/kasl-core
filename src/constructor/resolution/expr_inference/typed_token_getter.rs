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
    ConstructorError, ConstructorErrorType, ExprToken, ExprTokenKind, LiteralBind, Program,
    SymbolPath, SymbolTable,
};

pub enum TypedToken {
    Value(SymbolPath), // The type of the value
    PrefixOperator(String),
    InfixOperator(String),
    LParen,
    RParen,
}

/// Infer the type of each token in the expression and convert them to TypedTokens.
pub fn get_typed_tokens(
    program: &Program,
    expr: &[ExprToken],
    symbol_table: &SymbolTable,
) -> Result<Vec<TypedToken>, ConstructorError> {
    let mut expr_iter = expr.iter().peekable();
    let mut result: Vec<TypedToken> = Vec::new();

    while let Some(token) = expr_iter.next() {
        match &token.kind {
            ExprTokenKind::IntLiteral(_) => match &program.int_literal_type {
                Some(int_literal_type) => result.push(TypedToken::Value(int_literal_type.clone())),
                None => {
                    return Err(ConstructorError {
                        error_type: ConstructorErrorType::MissingLiteralBind(
                            LiteralBind::IntLiteral,
                        ),
                        position: token.range,
                    });
                }
            },

            ExprTokenKind::FloatLiteral(_) => match &program.float_literal_type {
                Some(float_literal_type) => {
                    result.push(TypedToken::Value(float_literal_type.clone()))
                }
                None => {
                    return Err(ConstructorError {
                        error_type: ConstructorErrorType::MissingLiteralBind(
                            LiteralBind::FloatLiteral,
                        ),
                        position: token.range,
                    });
                }
            },

            ExprTokenKind::BoolLiteral(_) => match &program.bool_literal_type {
                Some(bool_literal_type) => {
                    result.push(TypedToken::Value(bool_literal_type.clone()))
                }
                None => {
                    return Err(ConstructorError {
                        error_type: ConstructorErrorType::MissingLiteralBind(
                            LiteralBind::BoolLiteral,
                        ),
                        position: token.range,
                    });
                }
            },

            ExprTokenKind::Identifier(parser_path) => {
                let symbol_type = program.get_symbol_type(&parser_path, symbol_table, token)?;
                result.push(TypedToken::Value(symbol_type));
            }

            ExprTokenKind::FuncCall {
                path: func_parser_path,
                args: _,
            } => {
                let func_type = program.get_func_type(func_parser_path, symbol_table, token)?;
                result.push(TypedToken::Value(func_type));
            }

            ExprTokenKind::Operator(operator_symbol) => {
                let last_token = result.last();
                let operator_token = handle_operator_resolution(operator_symbol, last_token);
                result.push(operator_token);
            }

            ExprTokenKind::LParen => result.push(TypedToken::LParen),

            ExprTokenKind::RParen => result.push(TypedToken::RParen),
        }
    }

    Ok(result)
}

fn handle_operator_resolution(
    operator_symbol: &String,
    last_token: Option<&TypedToken>,
) -> TypedToken {
    // Whether the operator is infix or prefix can be determined by the last token
    let is_infix = match last_token {
        Some(TypedToken::Value(_)) | Some(TypedToken::RParen) => true,
        None
        | Some(TypedToken::LParen)
        | Some(TypedToken::InfixOperator(_))
        | Some(TypedToken::PrefixOperator(_)) => false,
    };

    if is_infix {
        return TypedToken::InfixOperator(operator_symbol.clone());
    } else {
        return TypedToken::PrefixOperator(operator_symbol.clone());
    }
}
