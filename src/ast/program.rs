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
    Function, InputVar, OutputVar, Scope, StateVar, SymbolPath, SymbolPathComponent, TypeDef,
};

pub struct Program<'a> {
    pub main_func: Option<Function<'a>>,
    pub funcs: Vec<Function<'a>>,
    pub types: Vec<TypeDef<'a>>,
    pub states: Vec<StateVar<'a>>,
    pub inputs: Vec<InputVar<'a>>,
    pub outputs: Vec<OutputVar<'a>>,
}

impl<'a> Program<'a> {
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

    /// Get a type definition by its path.
    pub fn get_type_def_by_path(&'a self, symbol_path: &SymbolPath) -> Option<&'a TypeDef<'a>> {
        let (last, parent) = symbol_path.components.split_last()?;
        let parent_scope = self.get_to_deepest_scope(parent)?;
        match last {
            SymbolPathComponent::TypeDef(name) => parent_scope.get_type_def(name),
            _ => None,
        }
    }

    /// Get an **immutable** reference to the scope which belongs to the last component of the given vector of the symbol path component.
    pub fn get_to_deepest_scope(
        &'a self,
        path_components: &[SymbolPathComponent],
    ) -> Option<&'a dyn Scope<'a>> {
        let mut current_scope: &dyn Scope<'a> = self;

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
        &'a mut self,
        path_components: &[SymbolPathComponent],
    ) -> Option<&'a mut dyn Scope<'a>> {
        let mut current_scope: &mut dyn Scope<'a> = self;

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

impl<'a> Scope<'a> for Program<'a> {
    fn get_func(&self, name: &str) -> Option<&Function<'a>> {
        self.funcs.iter().find(|f| f.name == name)
    }

    fn get_func_mut(&mut self, name: &str) -> Option<&mut Function<'a>> {
        self.funcs.iter_mut().find(|f| f.name == name)
    }

    fn get_type_def(&self, name: &str) -> Option<&TypeDef<'a>> {
        self.types.iter().find(|t| t.name == name)
    }

    fn get_type_def_mut(&mut self, name: &str) -> Option<&mut TypeDef<'a>> {
        self.types.iter_mut().find(|t| t.name == name)
    }

    fn get_state(&self, name: &str) -> Option<&StateVar<'a>> {
        self.states.iter().find(|s| s.name == name)
    }

    fn get_state_mut(&mut self, name: &str) -> Option<&mut StateVar<'a>> {
        self.states.iter_mut().find(|s| s.name == name)
    }

    fn get_input(&self, name: &str) -> Option<&InputVar<'a>> {
        self.inputs.iter().find(|i| i.name == name)
    }

    fn get_input_mut(&mut self, name: &str) -> Option<&mut InputVar<'a>> {
        self.inputs.iter_mut().find(|i| i.name == name)
    }

    fn get_output(&self, name: &str) -> Option<&OutputVar<'a>> {
        self.outputs.iter().find(|o| o.name == name)
    }

    fn get_output_mut(&mut self, name: &str) -> Option<&mut OutputVar<'a>> {
        self.outputs.iter_mut().find(|o| o.name == name)
    }
}
