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
    MAIN_FUNCTION_NAME, ParserTopLevelStmtKind, SymbolTable,
    error::{ErrorCollector, Ph},
    symbol_path,
};

/// Check for errors in the given Program and SymbolTable.
pub fn validate(ec: &mut ErrorCollector, symbol_table: &SymbolTable) {
    // Check if the main function exists
    let main_func_path = symbol_path![MAIN_FUNCTION_NAME.to_string()];
    let main_func_id = symbol_table.get_id_by_path(&main_func_path);
    if main_func_id.is_none() {
        ec.no_main_func(Ph::Validation);
    } else if let Some(main_stmt) =
        symbol_table.get_statement_by_id(main_func_id.unwrap().first().unwrap())
    {
        // If the `main` statement exists, check if it is a function
        if !matches!(main_stmt.kind, ParserTopLevelStmtKind::FuncDecl { .. }) {
            ec.main_stmt_not_func(main_stmt.range, Ph::Validation);
        }
    }
}
