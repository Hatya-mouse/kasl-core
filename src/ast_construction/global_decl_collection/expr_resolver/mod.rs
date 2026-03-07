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

mod recursive_resolver;

use crate::{
    Expr, OperatorContext, ScopeID, ScopeRegistry,
    error::ErrorCollector,
    symbol_table::FunctionContext,
    type_registry::{ResolvedType, TypeRegistry},
};

pub struct ExpressionResolver<'a> {
    pub ec: &'a mut ErrorCollector,
    pub op_ctx: &'a OperatorContext,
    pub func_ctx: &'a FunctionContext,
    pub scope_registry: &'a ScopeRegistry,
    pub type_registry: &'a TypeRegistry,
    pub current_scope: ScopeID,
}

impl ExpressionResolver<'_> {
    pub fn resolve(&mut self, expr: Expr<()>) -> Option<Expr<ResolvedType>> {
        self.resolve_recursively(expr)
    }
}
