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

use crate::{InfixOperator, InfixOperatorProperties, PrefixOperator, SymbolID};
use std::collections::HashMap;

#[derive(Debug)]
pub struct OperatorContext {
    pub infix_operators: HashMap<SymbolID, InfixOperator>,
    pub prefix_operators: HashMap<SymbolID, PrefixOperator>,
    pub infix_operator_properties: HashMap<String, InfixOperatorProperties>,
}

impl OperatorContext {
    pub fn register_infix_func(&mut self, infix: InfixOperator, id: SymbolID) {
        self.infix_operators.insert(id, infix);
    }

    pub fn register_prefix_func(&mut self, prefix: PrefixOperator, id: SymbolID) {
        self.prefix_operators.insert(id, prefix);
    }

    pub fn register_infix_properties(
        &mut self,
        symbol: String,
        properties: InfixOperatorProperties,
    ) {
        self.infix_operator_properties.insert(symbol, properties);
    }
}
