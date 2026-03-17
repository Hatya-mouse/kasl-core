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

use std::path::PathBuf;

use crate::{
    CompilationData,
    builtin::BuiltinRegistry,
    compilation_data::{CompilerState, ProgramContext},
    error::ErrorCollector,
};

#[derive(Default)]
pub struct KaslCompiler {
    ec: ErrorCollector,
    prog_ctx: ProgramContext,
    comp_data: CompilationData,
    comp_state: CompilerState,
    builtin_registry: BuiltinRegistry,
}

impl KaslCompiler {
    pub fn add_search_path(&mut self, path: PathBuf) {
        self.comp_state.child_search_paths.push(path);
    }
}
