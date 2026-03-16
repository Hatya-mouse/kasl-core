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
/// Builds a Block which contains ScopeID from a list of statements.
mod scope_block_builder;
mod stmt_builder;

use crate::{
    CompilationData, NameSpaceID, ScopeID, builtin::BuiltinRegistry,
    compilation_data::ProgramContext, error::ErrorCollector, scope_manager::ScopeGraph,
    type_registry::ResolvedType,
};

/// Builds a statements from raw parser statements.
/// Should not be reused across multiple blocks.
pub struct BlockStmtBuilder<'a> {
    ec: &'a mut ErrorCollector,
    prog_ctx: &'a mut ProgramContext,
    comp_data: &'a CompilationData,
    builtin_registry: &'a BuiltinRegistry,
    scope_graph: &'a mut ScopeGraph,

    scope_id: ScopeID,
    namespace_id: NameSpaceID,
    expected_return_type: ResolvedType,
}

impl<'a> BlockStmtBuilder<'a> {
    pub fn new(
        ec: &'a mut ErrorCollector,
        prog_ctx: &'a mut ProgramContext,
        comp_data: &'a CompilationData,
        builtin_registry: &'a BuiltinRegistry,
        scope_graph: &'a mut ScopeGraph,
        scope_id: ScopeID,
        namespace_id: NameSpaceID,
        expected_return_type: ResolvedType,
    ) -> Self {
        Self {
            ec,
            prog_ctx,
            comp_data,
            builtin_registry,
            scope_graph,
            scope_id,
            namespace_id,
            expected_return_type,
        }
    }

    pub fn mark_name_used(&mut self, name: &str) {
        // Mark the name as used in the namespace
        self.prog_ctx
            .namespace_registry
            .mark_name_used(&self.namespace_id, name);
    }
}
