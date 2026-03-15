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
    backend::func_translator::FuncTranslator,
    scope_manager::{BlueprintItem, IOBlueprint},
};
use cranelift::prelude::{InstBuilder, MemFlags};
use cranelift_codegen::ir;

impl FuncTranslator<'_> {
    pub fn load_blueprint_access(&mut self, blueprint: &IOBlueprint, entry_block: ir::Block) {
        // Get the pointer to the pointer array
        let block_params = self.builder.block_params(entry_block);
        let input_ptr_ptr = block_params[0];
        let output_ptr_ptr = block_params[1];
        let state_ptr_ptr = block_params[2];

        // Get the type of a pointer
        let pointer_type = self.type_converter.pointer_type();

        // Loop over the inputs, outputs and states and load them
        // INPUTS
        let mut input_offset: usize = 0;
        for input_item in blueprint.get_inputs() {
            self.load_blueprint_item(pointer_type, input_ptr_ptr, input_item, input_offset);
            input_offset += input_item.size;
        }

        // OUTPUTS
        let mut output_offset: usize = 0;
        for output_item in blueprint.get_outputs() {
            self.load_blueprint_item(pointer_type, output_ptr_ptr, output_item, output_offset);
            output_offset += output_item.size;
        }

        // STATES
        let mut state_offset: usize = 0;
        for state_item in blueprint.get_states() {
            self.load_blueprint_item(pointer_type, state_ptr_ptr, state_item, state_offset);
            state_offset += state_item.size;
        }
    }

    fn load_blueprint_item(
        &mut self,
        pointer_type: ir::Type,
        ptr_ptr: ir::Value,
        item: &BlueprintItem,
        offset: usize,
    ) -> ir::Value {
        // Get the pointer to the input value by the pointer to the pointers
        let ptr = self
            .builder
            .ins()
            .load(pointer_type, MemFlags::new(), ptr_ptr, offset as i32);

        // Load the input value
        self.builder.ins().load(
            self.type_converter.convert(&item.value_type),
            MemFlags::new(),
            ptr,
            0,
        )
    }
}
