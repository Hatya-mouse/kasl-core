//
//  Copyright 2025-2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

mod builder;
mod flow_node;

pub use builder::FlowGraphBuilder;
pub use flow_node::FlowNode;

use std::collections::HashMap;
use std::fmt::Display;

/// This is a Control Flow Graph (CFG),
/// which is used to check if the all paths have return statement.
#[derive(Default)]
pub struct FlowGraph {
    flow_nodes: HashMap<FlowID, FlowNode>,
    edges: HashMap<FlowID, Vec<FlowID>>,
    next_flow_id: usize,
}

impl FlowGraph {
    /// Generates a new flow ID.
    pub fn generate_flow_id(&mut self) -> FlowID {
        let id = FlowID(self.next_flow_id);
        self.next_flow_id += 1;
        id
    }

    /// Adds a node with the given has_return.
    pub fn add_flow_node(&mut self, has_return: bool) -> FlowID {
        let id = self.generate_flow_id();
        self.flow_nodes.insert(id, FlowNode { has_return });
        id
    }

    /// Change the has_return of the node.
    pub fn set_has_return(&mut self, id: &FlowID, has_return: bool) {
        if let Some(node) = self.flow_nodes.get_mut(id) {
            node.has_return = has_return;
        }
    }

    /// Adds an edge to the graph.
    pub fn add_flow_edge(&mut self, from: FlowID, to: FlowID) {
        self.edges.entry(from).or_default().push(to);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default, serde::Serialize)]
pub struct FlowID(usize);

impl Display for FlowID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
