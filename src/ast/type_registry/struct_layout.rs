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
    Expression, Range, SymbolID,
    type_registry::{ResolvedType, TypeRegistry},
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct StructLayout {
    /// The name of the struct.
    pub name: String,

    /// The fields of the struct.
    pub fields: Vec<StructField>,
    /// The IDs of the instance methods belonging to the struct.
    pub instance_methods: Vec<SymbolID>,
    /// The IDs of the static methods belonging to the struct.
    pub static_methods: Vec<SymbolID>,
    /// The map of field names to their indices in the `fields` vector.
    pub indices: HashMap<String, usize>,

    /// The map of field names to their offsets in bytes.
    pub field_offsets: Vec<u32>,
    /// The total size of the struct in bytes.
    pub total_size: u32,
    /// The alignment of the struct in bytes.
    pub alignment: u32,

    /// The range of the struct declaration in the source code.
    pub range: Range,
}

impl StructLayout {
    pub fn compute_layout(&mut self, type_registry: &TypeRegistry) {
        let mut offset = 0;
        let mut max_alignment = 1;

        for field in &mut self.fields {
            // Get the size and alignment of the field's type
            let size = type_registry.get_type_size(&field.value_type);
            let alignment = type_registry.get_type_alignment(&field.value_type);
            // If the alignment is greater than the max_alignment, update it
            if alignment > max_alignment {
                max_alignment = alignment;
            }
            // Align the offset to the field's alignment
            offset = (offset + (alignment - 1)) & !(alignment - 1);
            // Push the offset to the field_offsets vector
            self.field_offsets.push(offset);
            offset += size;
        }

        self.total_size = offset;
        self.alignment = max_alignment;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructField {
    pub name: String,
    pub value_type: ResolvedType,
    pub def_val: Expression,
    pub range: Range,
}
