//
// Copyright 2025-2026 Shuntaro Kasatani
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
    FuncParam, ParserFuncParam, Program, Range, SymbolTable,
    error::{ErrorCollector, Phase},
    resolution::expr_inference::ExprTreeBuilder,
};

pub fn resolve_param(
    ec: &mut ErrorCollector,
    program: &mut Program,
    symbol_table: &SymbolTable,
    symbol: &str,
    param: &ParserFuncParam,
    decl_range: Range,
) -> Option<FuncParam> {
    if let Some(value_type) = &param.value_type {
        // If the symbol has a type annotation, use it
        // Get the operand type path
        let operand_type = match program.resolve_type_def_parser_path(&value_type) {
            Some(operand_type) => operand_type,
            None => {
                ec.operator_not_found(decl_range, Phase::TypeResolution, symbol);
                return None;
            }
        };

        Some(FuncParam {
            label: param.label.clone(),
            name: param.name.clone(),
            value_type: Some(operand_type),
            def_val: None,
        })
    } else if let Some(def_val) = &param.def_val {
        // If the symbol does not have a type annotation, infer it from the expression
        let expr = match program.build_expr_tree_from_raw_tokens(ec, def_val, symbol_table) {
            Some(expr) => expr,
            None => return None,
        };
        let operand_type = match expr.get_type(ec, program, decl_range) {
            Some(operand_type) => operand_type,
            None => return None,
        };

        // Construct the prefix operator
        Some(FuncParam {
            label: param.label.clone(),
            name: param.name.clone(),
            value_type: Some(operand_type),
            def_val: Some(Box::new(expr)),
        })
    } else {
        // If the symbol does not have a type annotation or default value, throw an error
        ec.missing_type_annotation(decl_range, Phase::TypeResolution, &param.name);
        None
    }
}
