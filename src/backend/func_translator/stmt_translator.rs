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

use crate::{Statement, backend::func_translator::FuncTranslator};

impl FuncTranslator<'_> {
    pub fn translate_stmt(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Block { block } => {}
            Statement::LocalVar { var_id } => {}
            Statement::LocalConst { var_id } => {}
            Statement::Assign { target, value } => {}
            Statement::Expression { expr } => {}
            Statement::If {
                main,
                else_ifs,
                else_block,
            } => {}
            Statement::Return { value } => {}
        }
    }
}
