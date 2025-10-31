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

use crate::{
    DependencyGraphEdge, DependencyGraphNode, ExprToken, ExprTokenKind, SymbolPath, SymbolTable,
};

pub fn build_var_graph(
    graph: &mut Vec<DependencyGraphEdge>,
    root_symbol_table: &SymbolTable,
    var_path: SymbolPath,
    def_val: &Vec<ExprToken>,
) {
    for expr in def_val {
        match &expr.kind {
            ExprTokenKind::Identifier(path) => {
                let from_node = DependencyGraphNode::new(var_path.clone());

                let to_path = root_symbol_table.resolve_path(path);
                let to_node = DependencyGraphNode::new(to_path);

                let edge = DependencyGraphEdge::new(from_node, to_node);
                graph.push(edge);
            }
            _ => (),
        }
    }
}
