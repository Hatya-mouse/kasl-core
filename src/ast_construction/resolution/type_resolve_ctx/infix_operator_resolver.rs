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
    InfixOperator, InfixOperatorProperties, ParserFuncParam, Range, SymbolPath, error::Ph,
    name_space::ParserStmtID, resolution::TypeResolveCtx,
};

impl<'a> TypeResolveCtx<'a> {
    pub fn resolve_infix_func(
        &mut self,
        symbol: &str,
        symbol_id: &ParserStmtID,
        params: &[ParserFuncParam],
        return_type: &SymbolPath,
        decl_range: Range,
    ) {
        let Some(path) = self.symbol_table.get_path_by_id(symbol_id) else {
            return;
        };

        // Get the return type id
        let resolved_return_type = match self
            .type_registry
            .resolve_type_path(self.name_space, return_type)
        {
            Some(resolved_path) => resolved_path,
            None => {
                self.ec
                    .type_not_found(decl_range, Ph::TypeResolution, &return_type.to_string());
                return;
            }
        };

        // Check if the function has two parameters
        if params.len() != 2 {
            self.ec
                .invalid_param_numbers_for_infix(decl_range, Ph::TypeResolution, params.len());
            return;
        }

        // Resolve the parameters
        let lhs = match self.resolve_param(&params[0]) {
            Some(operand) => operand,
            None => return,
        };
        let rhs = match self.resolve_param(&params[1]) {
            Some(operand) => operand,
            None => return,
        };

        // Ensure that the parameters don't have any default value
        if lhs.def_val.is_some() {
            self.ec
                .op_def_val(decl_range, Ph::TypeResolution, &lhs.name);
        }
        if rhs.def_val.is_some() {
            self.ec
                .op_def_val(decl_range, Ph::TypeResolution, &rhs.name);
        }

        // Once we've got the types, we can get the exact operator
        let infix = InfixOperator {
            symbol: symbol.to_string(),
            lhs,
            rhs,
            return_type: resolved_return_type,
            body: Vec::new(),
            range: decl_range,
        };

        // Register the infix operator
        let id = self.name_space.register_path(path.clone());
        self.op_ctx.register_infix_func(infix, id);
    }

    pub fn register_infix_define(&mut self, symbol: &str, properties: InfixOperatorProperties) {
        self.op_ctx
            .register_infix_properties(symbol.to_string(), properties);
    }
}
