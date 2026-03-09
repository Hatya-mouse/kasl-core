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

mod builders;
mod stmt_builder;

use crate::{
    NameSpace, OperatorContext, ScopeRegistry,
    error::ErrorCollector,
    symbol_table::{FuncBodyMap, FunctionContext},
    type_registry::TypeRegistry,
};

pub struct StatementBuilder<'a> {
    ec: &'a mut ErrorCollector,
    name_space: &'a mut NameSpace,
    type_registry: &'a TypeRegistry,
    func_ctx: &'a mut FunctionContext,
    func_body_map: &'a FuncBodyMap,
    op_ctx: &'a OperatorContext,
    scope_registry: &'a mut ScopeRegistry,
}

impl<'a> StatementBuilder<'a> {
    pub fn new(
        ec: &'a mut ErrorCollector,
        name_space: &'a mut NameSpace,
        type_registry: &'a TypeRegistry,
        func_ctx: &'a mut FunctionContext,
        func_body_map: &'a FuncBodyMap,
        op_ctx: &'a OperatorContext,
        scope_registry: &'a mut ScopeRegistry,
    ) -> Self {
        Self {
            ec,
            name_space,
            type_registry,
            func_ctx,
            func_body_map,
            op_ctx,
            scope_registry,
        }
    }

    pub fn build_all(&mut self) {
        for func_id in self.func_ctx.func_ids() {
            self.build_func_body(func_id);
        }
    }
}
