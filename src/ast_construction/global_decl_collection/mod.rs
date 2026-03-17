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

mod resolvers;
mod stmt_process;

pub use resolvers::FuncDeclInfo;

use crate::{
    CompilationData, NameSpaceID, ParserDeclStmt,
    builtin::BuiltinRegistry,
    compilation_data::{CompilerState, ProgramContext},
    error::ErrorCollector,
};

pub struct GlobalDeclCollector<'a> {
    ec: &'a mut ErrorCollector,
    prog_ctx: &'a mut ProgramContext,
    comp_data: &'a mut CompilationData,
    comp_state: &'a CompilerState,
    builtin_registry: &'a BuiltinRegistry,

    current_namespace: NameSpaceID,
}

impl<'a> GlobalDeclCollector<'a> {
    pub fn new(
        ec: &'a mut ErrorCollector,
        prog_ctx: &'a mut ProgramContext,
        comp_data: &'a mut CompilationData,
        comp_state: &'a CompilerState,
        builtin_registry: &'a BuiltinRegistry,
        current_namespace: NameSpaceID,
    ) -> Self {
        Self {
            ec,
            prog_ctx,
            comp_data,
            comp_state,
            builtin_registry,
            current_namespace,
        }
    }

    pub fn process(&mut self, decl_stmts: &'a [ParserDeclStmt]) {
        for stmt in decl_stmts.iter() {
            println!("Stmt: {:?}", stmt);
            self.process_stmt(stmt);
        }
    }

    pub fn mark_name_used(&mut self, name: &str) {
        // Mark the name as used in the namespace
        self.prog_ctx
            .namespace_registry
            .mark_name_used(&self.current_namespace, name);
    }
}
