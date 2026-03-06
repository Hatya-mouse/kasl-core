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

use crate::{Function, SymbolID, type_registry::ResolvedType};
use std::collections::HashMap;

#[derive(Debug)]
pub struct FunctionContext {
    pub funcs: HashMap<SymbolID, Function>,
}

impl FunctionContext {
    pub fn get_type(&self, symbol_id: &SymbolID) -> Option<ResolvedType> {
        self.funcs
            .get(symbol_id)
            .and_then(|func| func.return_type.clone())
    }

    pub fn register_func(&mut self, func: Function, id: SymbolID) {
        self.funcs.insert(id, func);
    }
}
