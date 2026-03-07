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

use crate::{FuncCallArg, VariableID, type_registry::ResolvedType};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    IntLiteral(i32),
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
    Identifier {
        id: VariableID,
        value_type: ResolvedType,
    },
    MemberAccess {
        lhs: Box<Expression>,
        offset: usize,
        value_type: ResolvedType,
    },
    FuncCall {
        id: VariableID,
        args: Vec<FuncCallArg>,
        return_type: ResolvedType,
    },
}
