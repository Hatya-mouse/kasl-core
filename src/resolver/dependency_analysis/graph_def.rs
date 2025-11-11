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

use crate::SymbolPath;
use std::collections::HashMap;

pub struct DependencyGraph {
    pub nodes: HashMap<SymbolPath, DependencyGraphNode>,
    pub edges: Vec<DependencyGraphEdge>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        DependencyGraph {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: DependencyGraphNode) {
        self.nodes.insert(node.name.clone(), node);
    }

    pub fn add_edge(&mut self, from: &SymbolPath, to: &SymbolPath) {
        self.edges.push(DependencyGraphEdge {
            from: from.clone(),
            to: to.clone(),
        });
    }

    pub fn node(&self, name: &SymbolPath) -> Option<&DependencyGraphNode> {
        self.nodes.get(name)
    }

    pub fn edge(&self, from: &SymbolPath, to: &SymbolPath) -> Option<&DependencyGraphEdge> {
        self.edges
            .iter()
            .find(|edge| edge.from == *from && edge.to == *to)
    }

    pub fn get_edge_nodes(
        &self,
        edge: &DependencyGraphEdge,
    ) -> Option<(&DependencyGraphNode, &DependencyGraphNode)> {
        self.node(&edge.from)
            .and_then(|from| self.node(&edge.to).map(|to| (from, to)))
    }
}

pub struct DependencyGraphNode {
    name: SymbolPath,
}

impl DependencyGraphNode {
    pub fn new(name: SymbolPath) -> Self {
        DependencyGraphNode { name }
    }
}

impl Clone for DependencyGraphNode {
    fn clone(&self) -> Self {
        DependencyGraphNode {
            name: self.name.clone(),
        }
    }
}

/// Represents an edge in a dependency graph.
///
/// # Example
/// Edge `A -> B` means that "A depends on B", therefore B must be resolved before A.
pub struct DependencyGraphEdge {
    pub from: SymbolPath,
    pub to: SymbolPath,
}

impl DependencyGraphEdge {
    pub fn new(from: SymbolPath, to: SymbolPath) -> Self {
        DependencyGraphEdge { from, to }
    }
}
