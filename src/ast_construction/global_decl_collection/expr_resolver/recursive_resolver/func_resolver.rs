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
    Expr, ExprKind, FuncCallArg, Function, Range,
    global_decl_collection::expr_resolver::ExpressionResolver, symbol_table::NoTypeFuncCallArg,
    type_registry::ResolvedType,
};

impl ExpressionResolver<'_> {
    pub fn resolve_func(
        &mut self,
        name: String,
        no_type_args: Vec<NoTypeFuncCallArg>,
        range: Range,
    ) -> Option<Expr<ResolvedType>> {
        // Get a reference to the function
        let func_id = self.func_ctx.get_global_func_by_name(&name)?;
        let func = self.func_ctx.get_func(&func_id)?;

        let args = self.resolve_func_call_args(&func, &no_type_args)?;

        let Some(return_type) = &func.return_type else {
            self.ec.no_return_func_in_expr(range, &name);
            return None;
        };

        Some(Expr::new(
            ExprKind::FuncCall {
                name,
                id: Some(func_id),
                no_type_args,
                args: Some(args),
            },
            return_type.clone(),
            range,
        ))
    }

    pub fn resolve_func_call_args(
        &mut self,
        func: &Function,
        no_type_args: &[NoTypeFuncCallArg],
    ) -> Option<Vec<FuncCallArg>> {
    }
}
