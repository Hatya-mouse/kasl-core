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

mod io_blueprint;
mod scope;
mod scope_graph;
mod scope_var;

pub use io_blueprint::IOBlueprint;
pub use scope::Scope;
pub use scope_graph::ScopeGraph;
pub use scope_var::{InputAttribute, ScopeVar, VariableKind};

use crate::{Range, VariableID};
use std::collections::{HashMap, HashSet};

/// ScopeRegistry manages scopes and variables belonging to them.
/// It only manages the top-level variables and local variables,
/// and doesn't manage the struct fields.
#[derive(Debug, serde::Serialize)]
pub struct ScopeRegistry {
    pub scopes: HashMap<ScopeID, Scope>,
    variables: HashMap<VariableID, ScopeVar>,
    global_scope_id: ScopeID,
    next_scope_id: usize,
    next_variable_id: usize,
}

impl Default for ScopeRegistry {
    fn default() -> Self {
        let mut manager = Self {
            scopes: HashMap::new(),
            variables: HashMap::new(),
            global_scope_id: ScopeID(0),
            next_scope_id: 0,
            next_variable_id: 0,
        };
        // Create the global scope
        manager.global_scope_id = manager.create_scope(None, Range::zero());
        manager
    }
}

impl ScopeRegistry {
    /// Returns the ID of the global scope.
    pub fn get_global_scope_id(&self) -> ScopeID {
        self.global_scope_id
    }

    /// Returns a mutable reference to the global scope.
    pub fn get_global_scope(&self) -> &Scope {
        self.scopes.get(&self.global_scope_id).unwrap()
    }

    /// Returns a reference to the scope with the given `ScopeID`.
    pub fn get_scope(&self, scope_id: &ScopeID) -> Option<&Scope> {
        self.scopes.get(scope_id)
    }

    /// Generates a new `ScopeID` for a new scope.
    pub fn generate_scope_id(&mut self) -> ScopeID {
        let id = ScopeID(self.next_scope_id);
        self.next_scope_id += 1;
        id
    }

    /// Generates a new `VariableID` for a new variable.
    pub fn generate_var_id(&mut self) -> VariableID {
        let id = VariableID::new(self.next_variable_id);
        self.next_variable_id += 1;
        id
    }

    /// Creates a new scope with the given parent scope.
    pub fn create_scope(&mut self, parent: Option<ScopeID>, range: Range) -> ScopeID {
        let id = self.generate_scope_id();
        let scope = Scope::new(parent, range);
        self.scopes.insert(id, scope);
        id
    }

    /// Looks up a variable by name in the current scope and its parents.
    pub fn lookup_var(&self, current_scope: ScopeID, name: &str) -> Option<&VariableID> {
        let mut target = Some(current_scope);
        while let Some(scope_id) = target {
            let scope = &self.scopes[&scope_id];
            if let Some(symbol_id) = scope.get_id_by_name(name) {
                return Some(symbol_id);
            }
            target = scope.parent;
        }
        None
    }

    /// Returns whether a variable with the given name exists in the current scope or its parents.
    pub fn has_var(&self, current_scope: ScopeID, name: &str) -> bool {
        let mut target = Some(current_scope);
        while let Some(scope_id) = target {
            let scope = &self.scopes[&scope_id];
            if scope.has_var(name) {
                return true;
            }
            target = scope.parent;
        }
        false
    }

    /// Returns a reference to the variable by ID.
    pub fn get_var_by_id(&self, id: &VariableID) -> Option<&ScopeVar> {
        self.variables.get(id)
    }

    /// Registers a variable in the scope registry.
    pub fn register_var(&mut self, var: ScopeVar, name: String, scope: ScopeID) -> VariableID {
        let variable_id = self.generate_var_id();
        let target_scope = self.scopes.get_mut(&scope).unwrap();
        target_scope.register_var(name, variable_id);
        self.variables.insert(variable_id, var);
        variable_id
    }

    /// Returns the vector of all scope IDs.
    pub fn all_scope_ids(&self) -> HashSet<ScopeID> {
        self.scopes.keys().copied().collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default, serde::Serialize)]
pub struct ScopeID(usize);
