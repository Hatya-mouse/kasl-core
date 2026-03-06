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
    NameSpace, RawSymbolTable,
    error::ErrorCollector,
    symbol_table::{FunctionContext, OperatorContext, VariableContext},
    type_registry::TypeRegistry,
};

pub struct TypeResolveCtx<'a> {
    pub ec: &'a mut ErrorCollector,
    pub name_space: &'a mut NameSpace,
    pub func_ctx: &'a mut FunctionContext,
    pub op_ctx: &'a mut OperatorContext,
    pub var_ctx: &'a mut VariableContext,
    pub type_registry: &'a TypeRegistry,
    pub symbol_table: &'a RawSymbolTable<'a>,
}

impl<'a> TypeResolveCtx<'a> {
    pub fn new(
        ec: &'a mut ErrorCollector,
        name_space: &'a mut NameSpace,
        func_ctx: &'a mut FunctionContext,
        op_ctx: &'a mut OperatorContext,
        var_ctx: &'a mut VariableContext,
        type_registry: &'a TypeRegistry,
        symbol_table: &'a RawSymbolTable<'a>,
    ) -> Self {
        TypeResolveCtx {
            ec,
            name_space,
            func_ctx,
            op_ctx,
            var_ctx,
            type_registry,
            symbol_table,
        }
    }
}
