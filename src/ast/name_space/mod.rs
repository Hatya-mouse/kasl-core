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

mod symbol_id;
mod symbol_path;

pub use symbol_id::{ParserStmtID, SymbolID};
pub use symbol_path::{SymbolPath, SymbolPathComponent};

use std::collections::HashMap;

#[derive(Debug)]
pub struct NameSpace {
    id_to_path: HashMap<SymbolID, SymbolPath>,
    path_to_id: HashMap<SymbolPath, Vec<SymbolID>>,
    next_id: usize,
}

impl NameSpace {
    pub fn new() -> Self {
        Self {
            id_to_path: HashMap::new(),
            path_to_id: HashMap::new(),
            next_id: 0,
        }
    }

    /// Returns a SymbolID for the given SymbolPath.
    pub fn get_id_by_path(&self, path: &SymbolPath) -> Option<&Vec<SymbolID>> {
        self.path_to_id.get(path)
    }

    /// Returns a SymbolPath for the given SymbolID.
    pub fn get_path_by_id(&self, id: &SymbolID) -> Option<&SymbolPath> {
        self.id_to_path.get(id)
    }

    /// Registers a new SymbolID for the given SymbolPath.
    pub fn register_path_with_id(&mut self, path: SymbolPath, id: SymbolID) {
        self.path_to_id.entry(path.clone()).or_default().push(id);
        self.id_to_path.insert(id, path);
    }

    /// Returns a next available SymbolID.
    pub fn generate_id(&mut self) -> SymbolID {
        let id = SymbolID::new(self.next_id);
        self.next_id += 1;
        id
    }

    /// Registers a next available SymbolID for the given SymbolPath, and returns the new ID.
    pub fn register_path(&mut self, path: SymbolPath) -> SymbolID {
        let id = self.generate_id();
        self.register_path_with_id(path, id);
        id
    }
}
