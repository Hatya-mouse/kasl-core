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
    Function, InputVar, OutputVar, ParserSymbolPath, Scope, StateVar, SymbolPath,
    SymbolPathComponent, TypeDef,
};

pub struct Program {
    pub main_func: Option<Function>,
    pub funcs: Vec<Function>,
    pub types: Vec<TypeDef>,
    pub states: Vec<StateVar>,
    pub inputs: Vec<InputVar>,
    pub outputs: Vec<OutputVar>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            main_func: None,
            funcs: Vec::new(),
            types: Vec::new(),
            states: Vec::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    /// Resolve ParserSymbolPath to obtain a SymbolPath to the TypeDef.
    pub fn resolve_type_def_parser_path(
        &self,
        parser_symbol_path: &ParserSymbolPath,
    ) -> Option<SymbolPath> {
        let mut current_scope: &dyn Scope = self;
        let mut complete_path = SymbolPath::new();

        // Loop through each component in the ParserSymbolPath
        for component in parser_symbol_path {
            // Check if the current scope contains a TypeDef with the given symbol
            match current_scope.get_type_def(&component.symbol) {
                Some(next_scope) => {
                    // If it does, push the TypeDef component to the complete path and update the current scope
                    let type_def_name = component.symbol.clone();
                    let new_component = SymbolPathComponent::TypeDef(type_def_name);
                    complete_path.push(new_component);
                    current_scope = next_scope;
                }
                _ => return None,
            }
        }

        Some(complete_path)
    }

    /// Get a immutable reference to the TypeDef by its path.
    pub fn get_type_def_by_path(&self, symbol_path: &SymbolPath) -> Option<&TypeDef> {
        let (last, parent) = symbol_path.components.split_last()?;
        let parent_scope = self.get_to_deepest_scope(parent)?;

        match last {
            SymbolPathComponent::TypeDef(name) => parent_scope.get_type_def(name),
            _ => None,
        }
    }

    /// Get an **immutable** reference to the scope which belongs to the last component of the given vector of the symbol path component.
    pub fn get_to_deepest_scope(
        &self,
        path_components: &[SymbolPathComponent],
    ) -> Option<&dyn Scope> {
        let mut current_scope: &dyn Scope = self;

        for comp in path_components {
            match comp {
                SymbolPathComponent::TypeDef(name) => {
                    current_scope = current_scope.get_type_def(name)?;
                }
                _ => return None,
            }
        }

        Some(current_scope)
    }

    /// Get a **mutable** reference to the scope which belongs to the last component of the given vector of the symbol path component.
    pub fn get_to_deepest_scope_mut(
        &mut self,
        path_components: &[SymbolPathComponent],
    ) -> Option<&mut dyn Scope> {
        let mut current_scope: &mut dyn Scope = self;

        for comp in path_components {
            match comp {
                SymbolPathComponent::TypeDef(name) => {
                    current_scope = current_scope.get_type_def_mut(name)?;
                }
                _ => return None,
            }
        }

        Some(current_scope)
    }
}

impl Scope for Program {
    fn get_func(&self, name: &str) -> Option<&Function> {
        self.funcs.iter().find(|f| f.name == name)
    }

    fn get_func_mut(&mut self, name: &str) -> Option<&mut Function> {
        self.funcs.iter_mut().find(|f| f.name == name)
    }

    fn get_type_def(&self, name: &str) -> Option<&TypeDef> {
        self.types.iter().find(|t| t.name == name)
    }

    fn get_type_def_mut(&mut self, name: &str) -> Option<&mut TypeDef> {
        self.types.iter_mut().find(|t| t.name == name)
    }

    fn get_state(&self, name: &str) -> Option<&StateVar> {
        self.states.iter().find(|s| s.name == name)
    }

    fn get_state_mut(&mut self, name: &str) -> Option<&mut StateVar> {
        self.states.iter_mut().find(|s| s.name == name)
    }

    fn get_input(&self, name: &str) -> Option<&InputVar> {
        self.inputs.iter().find(|i| i.name == name)
    }

    fn get_input_mut(&mut self, name: &str) -> Option<&mut InputVar> {
        self.inputs.iter_mut().find(|i| i.name == name)
    }

    fn get_output(&self, name: &str) -> Option<&OutputVar> {
        self.outputs.iter().find(|o| o.name == name)
    }

    fn get_output_mut(&mut self, name: &str) -> Option<&mut OutputVar> {
        self.outputs.iter_mut().find(|o| o.name == name)
    }
}
