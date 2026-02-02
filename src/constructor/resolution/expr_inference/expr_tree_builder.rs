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
    ConstructorError, ExprToken, Expression, Program, SymbolTable, get_typed_tokens,
    resolution::expr_inference::{build_expr_tree_from_rpn, rearrange_tokens_to_rpn},
};

pub trait ExprTreeBuilder<'a> {
    /// Build a typed Expression from `expr` tokens using the provided `SymbolTable`.
    /// Returns a ConstructorError if typing, RPN conversion, or tree construction fails.
    fn build_expr_tree_from_raw_tokens(
        &self,
        expr: &[ExprToken],
        symbol_table: &SymbolTable,
    ) -> Result<Expression, Vec<ConstructorError>>;
}

impl<'a> ExprTreeBuilder<'a> for Program {
    fn build_expr_tree_from_raw_tokens(
        &self,
        expr: &[ExprToken],
        symbol_table: &SymbolTable,
    ) -> Result<Expression, Vec<ConstructorError>> {
        // 1. Convert tokens to TypedToken so we can easily look up their types
        let typed_tokens = get_typed_tokens(self, symbol_table, expr).map_err(|err| vec![err])?;
        // 2. Rearrange tokens to get reverse polish notation
        let rpn_tokens = rearrange_tokens_to_rpn(self, typed_tokens).map_err(|err| vec![err])?;
        // 3. Evaluate the reverse polish notation to get the type of the expression
        let expr_tree = build_expr_tree_from_rpn(self, symbol_table, rpn_tokens)?;

        Ok(expr_tree)
    }
}
