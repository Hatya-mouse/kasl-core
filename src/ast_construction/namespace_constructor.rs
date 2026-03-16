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

use std::{collections::HashSet, path::PathBuf};

use crate::{
    CompilationData, NameSpaceID, ParserDeclStmt,
    builtin::BuiltinRegistry,
    compilation_data::{CompilerConfig, ConstructorState, ProgramContext},
    error::ErrorCollector,
    global_decl_collection::GlobalDeclCollector,
    kasl_parser,
    scope_manager::ScopeGraph,
};
use peg::{error::ParseError, str::LineCol};

/// Constructs a single namespace from a raw source code string.
pub struct NameSpaceConstructor<'a> {
    ec: ErrorCollector,
    prog_ctx: &'a mut ProgramContext,
    comp_data: CompilationData,
    comp_config: CompilerConfig,
    builtin_registry: BuiltinRegistry,
    scope_graph: ScopeGraph,

    constructor_state: ConstructorState,
    code: String,
    decl_stmts: Vec<ParserDeclStmt>,
    namespace_id: NameSpaceID,
}

impl<'a> NameSpaceConstructor<'a> {
    pub fn new(
        prog_ctx: &'a mut ProgramContext,
        comp_config: CompilerConfig,
        imported_paths: HashSet<PathBuf>,
        namespace_id: NameSpaceID,
    ) -> Self {
        let constructor_state = ConstructorState { imported_paths };

        Self {
            ec: ErrorCollector::default(),
            prog_ctx,
            comp_data: CompilationData::default(),
            comp_config,
            builtin_registry: BuiltinRegistry::default(),
            scope_graph: ScopeGraph::default(),
            constructor_state,
            code: String::new(),
            decl_stmts: Vec::new(),
            namespace_id,
        }
    }

    pub fn set_code(&mut self, code: &str) {
        self.code = code.to_string();
    }

    // --- PROCESS FUNCTIONS ---

    pub fn parse(&mut self) -> Result<(), ParseError<LineCol>> {
        self.decl_stmts = kasl_parser::parse(&self.code)?;
        Ok(())
    }

    pub fn collect_global_decls(&mut self) {
        let mut global_decl_collector = GlobalDeclCollector::new(
            &mut self.ec,
            self.prog_ctx,
            &mut self.comp_data,
            &self.comp_config,
            &self.builtin_registry,
            &mut self.scope_graph,
            &self.constructor_state,
            self.namespace_id,
        );
        global_decl_collector.process(&self.decl_stmts);
    }
}
