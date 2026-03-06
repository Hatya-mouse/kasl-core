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
    FuncCallArg, Range, SymbolID,
    error::{ErrorCollector, Phase},
    symbol_table::{FunctionContext, VariableContext},
    type_registry::{PrimitiveType, ResolvedType},
};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    IntLiteral(u32),
    FloatLiteral(f32),
    BoolLiteral(bool),
    PrefixOperator {
        operand: Box<Expression>,
        operand_type: ResolvedType,
        return_type: ResolvedType,
    },
    InfixOperator {
        lhs: Box<Expression>,
        lhs_type: ResolvedType,
        rhs: Box<Expression>,
        rhs_type: ResolvedType,
        return_type: ResolvedType,
    },
    Identifier(SymbolID),
    FuncCall {
        id: SymbolID,
        args: Vec<FuncCallArg>,
    },
}

impl Expression {
    pub fn get_type(
        &self,
        ec: &mut ErrorCollector,
        var_ctx: &VariableContext,
        func_ctx: &FunctionContext,
        error_range: Range,
    ) -> Option<ResolvedType> {
        match self {
            Expression::IntLiteral(_) => Some(ResolvedType::Primitive(PrimitiveType::Int)),
            Expression::FloatLiteral(_) => Some(ResolvedType::Primitive(PrimitiveType::Float)),
            Expression::BoolLiteral(_) => Some(ResolvedType::Primitive(PrimitiveType::Bool)),
            Expression::PrefixOperator { return_type, .. } => Some(return_type.clone()),
            Expression::InfixOperator { return_type, .. } => Some(return_type.clone()),
            Expression::Identifier(symbol_id) => match var_ctx.get_type(symbol_id) {
                Some(value_type) => Some(value_type),
                None => {
                    ec.comp_bug(
                        error_range,
                        Phase::TypeResolution,
                        &format!(
                            "Identifier with ID {} which should have been resolved",
                            symbol_id
                        ),
                    );
                    None
                }
            },
            Expression::FuncCall { id: func_id, .. } => match func_ctx.get_type(func_id) {
                Some(return_type) => Some(return_type),
                None => {
                    ec.comp_bug(
                        error_range,
                        Phase::TypeResolution,
                        &format!(
                            "Call to function with ID {} which should have been resolved",
                            func_id
                        ),
                    );
                    None
                }
            },
        }
    }
}
