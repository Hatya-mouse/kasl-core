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

use crate::{Expression, compilation::Translator, data::SymbolID};
use cranelift::prelude::*;

impl Translator<'_> {
    pub fn declare_variable(&self, name: &str, value_type: SymbolID, def_val: Value) -> Variable {
        let ir_type = todo!();
        let var = self.builder.declare_var(ir_type);
        self.builder.def_var(var, def_val);
        var
    }
}
