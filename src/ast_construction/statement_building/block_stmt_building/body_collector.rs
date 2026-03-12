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
    FunctionID, OperatorID, Range, ScopeID, error::Ph, statement_building::BlockStmtBuilder,
};

impl BlockStmtBuilder<'_> {
    pub fn build_func_body(&mut self, func_id: FunctionID) {
        let mut resolved_stmts = Vec::new();
        if let Some(body) = self.func_body_map.get_body(&func_id) {
            for stmt in body {
                // Get a reference to the function
                let Some(func) = self.comp_state.func_ctx.get_func(&func_id) else {
                    continue;
                };

                // Build the statements in the function
                let Some(resolved_stmt) =
                    self.build_stmt(stmt, func.block.scope_id, func.return_type)
                else {
                    continue;
                };
                resolved_stmts.push(resolved_stmt);
            }
        }

        // If the function has non-void return type, check if the function has a return for all paths
        if let Some(func) = self.comp_state.func_ctx.get_func(&func_id)
            && !func.return_type.is_void()
        {
            self.verify_return_for_func(func.block.scope_id, func.range);
        }

        // Set the statement to the block
        if let Some(func) = self.comp_state.func_ctx.get_func_mut(&func_id) {
            func.block.set_stmt(resolved_stmts);
        }
    }

    pub fn build_infix_body(&mut self, op_id: OperatorID) {
        let mut resolved_stmts = Vec::new();
        if let Some(body) = self.op_body_map.get_body(&op_id) {
            for stmt in body {
                // Get a reference to the operator
                let Some(op) = self.comp_state.op_ctx.get_infix_op(&op_id) else {
                    continue;
                };

                // Build the statements in the infix func
                let Some(resolved_stmt) = self.build_stmt(stmt, op.block.scope_id, op.return_type)
                else {
                    continue;
                };
                resolved_stmts.push(resolved_stmt);
            }
        }

        // Check if the operator has a return for all paths (Operators should have non-void return type)
        if let Some(op) = self.comp_state.op_ctx.get_infix_op(&op_id) {
            self.verify_return_for_func(op.block.scope_id, op.range);
        }

        // Set the statement to the block
        if let Some(op) = self.comp_state.op_ctx.get_infix_op_mut(&op_id) {
            op.block.set_stmt(resolved_stmts);
        }
    }

    pub fn build_prefix_body(&mut self, op_id: OperatorID) {
        // Get the ScopeID of the block
        let mut resolved_stmts = Vec::new();
        if let Some(body) = self.op_body_map.get_body(&op_id) {
            for stmt in body {
                // Get a reference to the operator
                let Some(op) = self.comp_state.op_ctx.get_prefix_op(&op_id) else {
                    continue;
                };

                // Build the statements in the prefix func
                let Some(resolved_stmt) = self.build_stmt(stmt, op.block.scope_id, op.return_type)
                else {
                    continue;
                };
                resolved_stmts.push(resolved_stmt);
            }
        }

        // Check if the operator has a return for all paths (Operators should have non-void return type)
        if let Some(op) = self.comp_state.op_ctx.get_prefix_op(&op_id) {
            self.verify_return_for_func(op.block.scope_id, op.range);
        }

        // Set the statement to the block
        if let Some(op) = self.comp_state.op_ctx.get_prefix_op_mut(&op_id) {
            op.block.set_stmt(resolved_stmts);
        }
    }

    pub fn build_postfix_body(&mut self, op_id: OperatorID) {
        // Get the ScopeID of the block
        let mut resolved_stmts = Vec::new();
        if let Some(body) = self.op_body_map.get_body(&op_id) {
            for stmt in body {
                // Get a reference to the operator
                let Some(op) = self.comp_state.op_ctx.get_postfix_op(&op_id) else {
                    continue;
                };

                // Build the statements in the postfix func
                let Some(resolved_stmt) = self.build_stmt(stmt, op.block.scope_id, op.return_type)
                else {
                    continue;
                };
                resolved_stmts.push(resolved_stmt);
            }
        }

        // Check if the operator has a return for all paths (Operators should have non-void return type)
        if let Some(op) = self.comp_state.op_ctx.get_postfix_op(&op_id) {
            self.verify_return_for_func(op.block.scope_id, op.range);
        }

        // Set the statement to the block
        if let Some(op) = self.comp_state.op_ctx.get_postfix_op_mut(&op_id) {
            op.block.set_stmt(resolved_stmts);
        }
    }

    fn verify_return_for_func(&mut self, func_scope_id: ScopeID, func_range: Range) {
        let has_return = *self.scope_has_return.entry(func_scope_id).or_insert(false);
        if !has_return {
            self.ec.missing_return(func_range, Ph::StatementCollection);
        }
    }
}
