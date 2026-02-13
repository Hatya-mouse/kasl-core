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

use crate::{FuncCallArg, Program, SymbolPath};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    IntLiteral(u32),
    FloatLiteral(f32),
    BoolLiteral(bool),
    PrefixOperator {
        operand: Box<Expression>,
        operand_type: SymbolPath,
        return_type: SymbolPath,
    },
    InfixOperator {
        lhs: Box<Expression>,
        lhs_type: SymbolPath,
        rhs: Box<Expression>,
        rhs_type: SymbolPath,
        return_type: SymbolPath,
    },
    Identifier(SymbolPath),
    FuncCall {
        path: SymbolPath,
        args: Vec<FuncCallArg>,
    },
}

impl Expression {
    pub fn get_type(&self, program: &Program) -> Option<SymbolPath> {
        match self {
            Expression::IntLiteral(_) => program.int_literal_type.clone(),
            Expression::FloatLiteral(_) => program.float_literal_type.clone(),
            Expression::BoolLiteral(_) => program.bool_literal_type.clone(),
            Expression::PrefixOperator { return_type, .. } => Some(return_type.clone()),
            Expression::InfixOperator { return_type, .. } => Some(return_type.clone()),
            Expression::Identifier(symbol_path) => Some(symbol_path.clone()),
            Expression::FuncCall { path, .. } => {
                if let Some(func) = program.get_func_by_path(path) {
                    func.return_type.clone()
                } else {
                    None
                }
            }
        }
    }
}
