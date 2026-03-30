//
//  Copyright 2025-2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

use crate::{
    ast::{FunctionID, compilation_data::ProgramContext, scope_manager::IOBlueprint},
    builtin::BuiltinRegistry,
};
use kasl_ir::ir::{IRBuilder, IRType};

#[derive(Default)]
pub struct Lowerer;

impl Lowerer {
    /// Lower the given program context to KASL-IR.
    pub fn lower(
        &self,
        prog_ctx: &ProgramContext,
        builtin_registry: &BuiltinRegistry,
        blueprint: &IOBlueprint,
        entry_point: &FunctionID,
    ) {
        // Create a ir builder
        let mut builder = IRBuilder::default();

        // Create an entry block and switch to the block
        // Add parameter for the input and output pointers
        // 1: input pointer
        // 2: output pointer
        // 3: state pointer
        // 4: whether to initialize the states
        let entry_block =
            builder.create_block(&[IRType::Ptr, IRType::Ptr, IRType::Ptr, IRType::I8]);
        builder.switch_to_block(entry_block);

        // Set the block as the entry block of the function
        builder.set_entry_block(entry_block);

        // Get the entry block parameters
        let params = builder.get_block_args(entry_block) else {
            return;
        };

        // Create a return block
        let return_block = builder.create_block(&[]);

        // Lower the program context to KASL-IR

        // Add return instruction to the return block
    }
}
