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
    ParserFuncParam, ParserSymbolPath, PrefixOperator, Program, Range, SymbolTable,
    error::{ErrorCollector, Phase},
    resolution::resolvers::param_resolver::resolve_param,
};

pub fn resolve_prefix_operator(
    ec: &mut ErrorCollector,
    program: &mut Program,
    symbol_table: &SymbolTable,
    symbol: &str,
    params: &[ParserFuncParam],
    return_type: &ParserSymbolPath,
    decl_range: Range,
) {
    // Get the return type path
    let return_type_path = match program.resolve_type_def_parser_path(return_type) {
        Some(return_type_path) => return_type_path,
        None => {
            ec.type_not_found(decl_range, Phase::TypeResolution, &return_type.to_string());
            return;
        }
    };

    // Resolve the operand
    let operand = match resolve_param(ec, program, symbol_table, symbol, &params[0], decl_range) {
        Some(operand) => operand,
        None => return,
    };

    // Construct the prefix operator
    let prefix = PrefixOperator {
        symbol: symbol.to_string(),
        operand,
        return_type: Some(return_type_path),
        body: Vec::new(),
    };
    program.register_prefix_func(prefix);
}
