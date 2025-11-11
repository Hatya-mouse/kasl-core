//
// Copyright 2025 Shuntaro Kasatani
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

use std::ops::Index;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct SymbolPath {
    components: Vec<SymbolPathComponent>,
}

impl SymbolPath {
    pub fn new() -> Self {
        SymbolPath {
            components: Vec::new(),
        }
    }

    pub fn push(&mut self, component: SymbolPathComponent) {
        self.components.push(component);
    }
}

impl Index<usize> for SymbolPath {
    type Output = SymbolPathComponent;

    fn index(&self, index: usize) -> &Self::Output {
        &self.components[index]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum SymbolPathComponent {
    CompInt,
    CompFloat,
    CompBool,
    Var(String),
    Func(String),
    TypeDef(String),
    FuncParam(String),
}

// Use this macro to create a SymbolPath from a simple list of components
// Example:
// ```
// symbol_path!["foo", "bar", "baz"];
// ```
#[macro_export]
macro_rules! symbol_path {
    ( $( $x:expr ),* $(,)? ) => {
        {
            let mut temp_path = $crate::ast::SymbolPath::new();
            $(
                let temp_val = $x;
                // Type check to ensure it's SymbolPathComponent
                let _: &$crate::ast::SymbolPathComponent = &temp_val;
                // Push the component to the vector
                temp_path.push(temp_val);
            )*
            temp_path
        }
    };
}
