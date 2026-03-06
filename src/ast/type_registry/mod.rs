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

mod primitive_type;
mod resolved_type;
mod struct_layout;

pub use primitive_type::PrimitiveType;
pub use resolved_type::ResolvedType;
pub use struct_layout::{StructField, StructLayout};

use crate::{NameSpace, SymbolID, SymbolPath};
use std::collections::HashMap;

#[derive(Debug)]
pub struct TypeRegistry {
    pub structs: HashMap<SymbolID, StructLayout>,
}

impl TypeRegistry {
    pub fn resolve_type_path(
        &self,
        name_space: &NameSpace,
        type_path: &SymbolPath,
    ) -> Option<ResolvedType> {
        if type_path.len() == 1 {
            if let Some(primitive_type) = PrimitiveType::from_str(&type_path.last().unwrap().symbol)
            {
                return Some(ResolvedType::Primitive(primitive_type));
            }
        }
        let id = name_space.get_id_by_path(type_path)?.first().unwrap();
        Some(ResolvedType::Struct(*id))
    }

    pub fn get_type_size(&self, type_id: &ResolvedType) -> u32 {
        match type_id {
            ResolvedType::Primitive(ty) => ty.size(),
            ResolvedType::Struct(id) => self.structs[id].total_size,
        }
    }

    pub fn get_type_alignment(&self, type_id: &ResolvedType) -> u32 {
        match type_id {
            ResolvedType::Primitive(ty) => ty.alignment(),
            ResolvedType::Struct(id) => self.structs[id].alignment,
        }
    }
}
