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

mod infix_operator;
mod postfix_operator;
mod prefix_operator;

pub use infix_operator::{InfixOperator, InfixOperatorProperties, OperatorAssociativity};
pub use postfix_operator::{PostfixOperator, PostfixOperatorProperties};
pub use prefix_operator::{PrefixOperator, PrefixOperatorProperties};

use crate::OperatorID;
use std::collections::HashMap;

#[derive(Debug)]
pub struct OperatorContext {
    infix_operator_properties: HashMap<String, InfixOperatorProperties>,
    infix_operators: HashMap<OperatorID, InfixOperator>,
    infix_ids: HashMap<String, OperatorID>,

    prefix_operator_properties: HashMap<String, PrefixOperatorProperties>,
    prefix_operators: HashMap<OperatorID, PrefixOperator>,
    prefix_ids: HashMap<String, OperatorID>,

    postfix_operator_properties: HashMap<String, PostfixOperatorProperties>,
    postfix_operators: HashMap<OperatorID, PostfixOperator>,
    postfix_ids: HashMap<String, OperatorID>,
}

impl OperatorContext {
    // -- REGISTER FUNCTIONS --

    pub fn register_infix_func(&mut self, infix: InfixOperator, id: OperatorID) {
        self.infix_operators.insert(id, infix);
    }

    pub fn register_infix_properties(
        &mut self,
        symbol: String,
        properties: InfixOperatorProperties,
    ) {
        self.infix_operator_properties.insert(symbol, properties);
    }

    pub fn register_prefix_func(&mut self, prefix: PrefixOperator, id: OperatorID) {
        self.prefix_operators.insert(id, prefix);
    }

    pub fn register_prefix_properties(
        &mut self,
        symbol: String,
        properties: PrefixOperatorProperties,
    ) {
        self.prefix_operator_properties.insert(symbol, properties);
    }

    pub fn register_postfix_func(&mut self, postfix: PostfixOperator, id: OperatorID) {
        self.postfix_operators.insert(id, postfix);
    }

    pub fn register_postfix_properties(
        &mut self,
        symbol: String,
        properties: PostfixOperatorProperties,
    ) {
        self.postfix_operator_properties.insert(symbol, properties);
    }

    // -- GETTER FUNCTIONS --

    pub fn get_infix_props(&self, symbol: &str) -> Option<&InfixOperatorProperties> {
        self.infix_operator_properties.get(symbol)
    }

    pub fn get_prefix_props(&self, symbol: &str) -> Option<&PrefixOperatorProperties> {
        self.prefix_operator_properties.get(symbol)
    }

    pub fn get_postfix_props(&self, symbol: &str) -> Option<&PostfixOperatorProperties> {
        self.postfix_operator_properties.get(symbol)
    }
}
