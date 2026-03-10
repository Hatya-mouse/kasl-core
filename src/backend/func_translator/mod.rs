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

mod stmt_translator;
mod translators;

use crate::{CompilationState, FunctionID};
use cranelift::prelude::FunctionBuilderContext;
use cranelift_jit::JITModule;

pub struct FuncTranslator<'a> {
    build_ctx: &'a mut FunctionBuilderContext,
    module: &'a JITModule,

    comp_state: &'a CompilationState,
}

impl<'a> FuncTranslator<'a> {
    pub fn new(
        build_ctx: &'a mut FunctionBuilderContext,
        module: &'a JITModule,
        comp_state: &'a CompilationState,
    ) -> Self {
        Self {
            build_ctx,
            module,
            comp_state,
        }
    }

    pub fn translate(&mut self, entry_point: &FunctionID) {}
}
