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
    ExprToken, ExprTokenKind, NameSpace, Range, VariableID,
    error::{ErrorCollector, Ph, Phase},
    resolution::expr_inference::SymbolTypeGetter,
    symbol_table::{FunctionContext, VariableContext},
    type_registry::{PrimitiveType, ResolvedType, TypeRegistry},
};

#[derive(Debug, Clone, PartialEq)]
pub struct TypedToken {
    pub kind: TypedTokenKind,
    pub range: Range,
}

impl TypedToken {
    pub fn new(kind: TypedTokenKind, range: Range) -> Self {
        TypedToken { kind, range }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypedTokenKind {
    Value {
        expr_token: ExprToken,
        value_type: ResolvedType,
    },
    PrefixOperator(String),
    InfixOperator(String),
    LParen,
    RParen,
}

/// Infer the type of each token in the expression and convert them to TypedTokens.
pub fn get_typed_tokens(
    ec: &mut ErrorCollector,
    name_space: &NameSpace,
    func_ctx: &FunctionContext,
    var_ctx: &VariableContext,
    expr: &[ExprToken],
) -> Option<Vec<TypedToken>> {
    let expr_iter = expr.iter().peekable();
    let mut result: Vec<TypedToken> = Vec::new();

    for token in expr_iter {
        match &token.kind {
            ExprTokenKind::IntLiteral(_) => {
                result.push(TypedToken {
                    kind: TypedTokenKind::Value {
                        expr_token: token.clone(),
                        value_type: ResolvedType::Primitive(PrimitiveType::Int),
                    },
                    range: token.range,
                });
            }

            ExprTokenKind::FloatLiteral(_) => {
                result.push(TypedToken {
                    kind: TypedTokenKind::Value {
                        expr_token: token.clone(),
                        value_type: ResolvedType::Primitive(PrimitiveType::Float),
                    },
                    range: token.range,
                });
            }

            ExprTokenKind::BoolLiteral(_) => {
                result.push(TypedToken {
                    kind: TypedTokenKind::Value {
                        expr_token: token.clone(),
                        value_type: ResolvedType::Primitive(PrimitiveType::Bool),
                    },
                    range: token.range,
                });
            }

            ExprTokenKind::Identifier(path) => {
                if let Some(symbol_type) = name_space
                    .get_id_by_path(path)
                    .and_then(|ids| var_ctx.get_type_of(ids.first().unwrap()))
                {
                    result.push(TypedToken::new(
                        TypedTokenKind::Value {
                            expr_token: token.clone(),
                            value_type: symbol_type,
                        },
                        token.range,
                    ));
                } else {
                    return None;
                }
            }

            ExprTokenKind::FuncCall { path, .. } => {
                if let Some(symbol_type) = name_space
                    .get_id_by_path(path)
                    .and_then(|ids| func_ctx.get_type_of(ids.first().unwrap()))
                {
                    result.push(TypedToken::new(
                        TypedTokenKind::Value {
                            expr_token: token.clone(),
                            value_type: symbol_type,
                        },
                        token.range,
                    ));
                } else {
                    return None;
                }
            }

            ExprTokenKind::Operator(operator_symbol) => {
                let last_token = result.last();
                let operator_token =
                    handle_operator_resolution(operator_symbol, token.range, last_token);
                result.push(operator_token);
            }

            ExprTokenKind::LParen => {
                result.push(TypedToken::new(TypedTokenKind::LParen, token.range))
            }

            ExprTokenKind::RParen => {
                result.push(TypedToken::new(TypedTokenKind::RParen, token.range))
            }
        }
    }

    Some(result)
}

fn handle_operator_resolution(
    operator_symbol: &str,
    operator_range: Range,
    last_token: Option<&TypedToken>,
) -> TypedToken {
    // Whether the operator is infix or prefix can be determined by the last token
    let is_infix = match last_token {
        Some(unwrapped_token) => matches!(
            unwrapped_token.kind,
            TypedTokenKind::Value {
                expr_token: _,
                value_type: _,
            } | TypedTokenKind::RParen
        ),
        None => false,
    };

    if is_infix {
        TypedToken::new(
            TypedTokenKind::InfixOperator(operator_symbol.to_string()),
            operator_range,
        )
    } else {
        TypedToken::new(
            TypedTokenKind::PrefixOperator(operator_symbol.to_string()),
            operator_range,
        )
    }
}
