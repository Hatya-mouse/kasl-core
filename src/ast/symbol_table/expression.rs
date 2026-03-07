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

use crate::{FuncCallArg, FunctionID, OperatorID, VariableID};

#[derive(Debug, PartialEq, Clone)]
pub struct Expr<T> {
    pub kind: ExprKind<T>,
    pub value_type: T,
}

impl<T> Expr<T> {
    pub fn new(kind: ExprKind<T>, value_type: T) -> Self {
        Self { kind, value_type }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExprKind<T> {
    IntLiteral(i32),
    FloatLiteral(f32),
    BoolLiteral(bool),
    InfixOp {
        symbol: String,
        operator: Option<OperatorID>,
        lhs: Box<Expr<T>>,
        rhs: Box<Expr<T>>,
    },
    PrefixOp {
        symbol: String,
        operator: Option<OperatorID>,
        operand: Box<Expr<T>>,
    },
    PostfixOp {
        symbol: String,
        operator: Option<OperatorID>,
        operand: Box<Expr<T>>,
    },
    Identifier {
        name: String,
        id: Option<VariableID>,
    },
    FuncCall {
        name: String,
        id: Option<FunctionID>,
        args: Option<Vec<FuncCallArg>>,
    },
    Chain {
        lhs: Box<Expr<T>>,
        access: MemberAccess,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum MemberAccess {
    Access {
        name: String,
        offset: Option<usize>,
    },
    FuncCall {
        name: String,
        args: Option<Vec<FuncCallArg>>,
    },
}
