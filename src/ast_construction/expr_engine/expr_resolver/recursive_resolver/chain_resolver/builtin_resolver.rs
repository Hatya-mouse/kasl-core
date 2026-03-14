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
    Expr, ExprKind, Range, error::Ph, expr_engine::ExpressionResolver, symbol_table::MemberAccess,
    type_registry::ResolvedType,
};

impl ExpressionResolver<'_> {
    pub fn resolve_builtin_call(
        &mut self,
        access: MemberAccess,
        range: Range,
    ) -> Option<Expr<ResolvedType>> {
        // Assume builtin func call is a func call
        match access {
            MemberAccess::Access { .. } => {
                self.ec.builtin_var_access(range, Ph::ExprEngine);
                None
            }
            MemberAccess::FuncCall {
                name, no_type_args, ..
            } => {
                // Get the BuiltinFuncID of the builtin function
                let Some(func_id) = self.builtin_registry.get_id_by_name(&name) else {
                    self.ec.builtin_func_not_found(range, Ph::ExprEngine, name);
                    return None;
                };
                // Get a reference to the function
                let func = self.builtin_registry.get_func_by_id(func_id)?;

                // Resolve the arguments
                let mut args = Vec::new();
                for (expected_type, no_type_arg) in func.params.iter().zip(no_type_args) {
                    let resolved_arg = self.resolve_recursively(no_type_arg.value)?;
                    // Check if the type of the argument matches the expected type
                    if &resolved_arg.value_type != expected_type {
                        self.ec.builtin_arg_type_mismatch(range, Ph::ExprEngine);
                    }

                    args.push(resolved_arg);
                }

                // Create an expression and return
                Some(Expr::new(
                    ExprKind::BuiltinFuncCall {
                        name,
                        id: *func_id,
                        args,
                    },
                    func.return_type,
                    range,
                ))
            }
        }
    }
}
