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
    Expr, ExprKind, Range, global_decl_collection::expr_resolver::ExpressionResolver,
    symbol_table::MemberAccess, type_registry::ResolvedType,
};

impl ExpressionResolver<'_> {
    pub fn resolve_chain(
        &mut self,
        lhs: Box<Expr<()>>,
        access: MemberAccess,
        range: Range,
    ) -> Option<Expr<ResolvedType>> {
        // Resolve the LHS expression
        let resolved_lhs = self.resolve(*lhs)?;

        // Resolve the access expression
        let (resolved_access, value_type) =
            self.resolve_member_access(&resolved_lhs.value_type, access, range)?;

        Some(Expr::new(
            ExprKind::Chain {
                lhs: Box::new(resolved_lhs),
                access: resolved_access,
            },
            value_type,
            range,
        ))
    }

    fn resolve_member_access(
        &mut self,
        lhs_type: &ResolvedType,
        access: MemberAccess,
        range: Range,
    ) -> Option<(MemberAccess, ResolvedType)> {
        match lhs_type {
            // If the LHS is a primitive type, the member access is invalid
            ResolvedType::Primitive(_) => {
                self.ec.member_access_on_primitive(range);
                None
            }

            // If the LHS is a struct type, get the offset of the field
            ResolvedType::Struct(struct_id) => match access {
                MemberAccess::Access { name, .. } => {
                    let struct_decl = self.type_registry.get_struct(&struct_id)?;
                    let field_index = struct_decl.get_field_index(&name)?;
                    let field = struct_decl.get_field_by_index(field_index)?;
                    let offset = struct_decl.get_offset_by_index(field_index)?;
                    Some((
                        MemberAccess::Access {
                            name,
                            offset: Some(offset),
                        },
                        field.value_type,
                    ))
                }

                MemberAccess::FuncCall {
                    name,
                    no_type_args,
                    args,
                } => {}
            },
        }
    }
}
