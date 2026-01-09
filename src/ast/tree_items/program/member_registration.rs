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
    ConstructorError, ConstructorErrorType, Function, Initializer, Program, Range, ScopeItemMut,
    ScopeVar, SymbolPath,
};

impl Program {
    /// Register a Function to the program **by its path**.
    pub fn register_func_by_path(
        &mut self,
        func: Function,
        to_path: &SymbolPath,
    ) -> Result<(), ConstructorError> {
        let target_scope = match self.get_to_deepest_scope_mut(&to_path.components) {
            Some(scope) => scope,
            None => {
                return Err(ConstructorError {
                    error_type: ConstructorErrorType::SymbolNotFound(Some(to_path.clone())),
                    position: Range::zero(),
                });
            }
        };

        match target_scope {
            ScopeItemMut::Program(prog) => prog.register_func(func),
            ScopeItemMut::TypeDef(td) => td.register_func(func),
            other => {
                return Err(unexpected_scope_error(
                    "register_func_by_path",
                    to_path,
                    &format!("{:?}", other),
                ));
            }
        }

        Ok(())
    }

    /// Register a Initializer to the program **by its path**.
    pub fn register_init_by_path(
        &mut self,
        init: Initializer,
        to_path: &SymbolPath,
    ) -> Result<(), ConstructorError> {
        let target_scope = match self.get_to_deepest_scope_mut(&to_path.components) {
            Some(scope) => scope,
            None => {
                return Err(ConstructorError {
                    error_type: ConstructorErrorType::SymbolNotFound(Some(to_path.clone())),
                    position: Range::zero(),
                });
            }
        };

        match target_scope {
            ScopeItemMut::TypeDef(td) => td.register_init(init),
            other => {
                return Err(unexpected_scope_error(
                    "register_init_by_path",
                    to_path,
                    &format!("{:?}", other),
                ));
            }
        }

        Ok(())
    }

    /// Register a ScopeVar to the program **by its path**.
    pub fn register_var_by_path(
        &mut self,
        var: ScopeVar,
        to_path: &SymbolPath,
    ) -> Result<(), ConstructorError> {
        let target_scope = match self.get_to_deepest_scope_mut(&to_path.components) {
            Some(scope) => scope,
            None => {
                return Err(ConstructorError {
                    error_type: ConstructorErrorType::SymbolNotFound(Some(to_path.clone())),
                    position: Range::zero(),
                });
            }
        };

        match target_scope {
            ScopeItemMut::TypeDef(td) => td.register_var(var),
            other => {
                return Err(unexpected_scope_error(
                    "register_var_by_path",
                    to_path,
                    &format!("{:?}", other),
                ));
            }
        }

        Ok(())
    }
}

fn unexpected_scope_error(func: &str, path: &SymbolPath, found: &str) -> ConstructorError {
    debug_assert!(
        false,
        "{} reached unexpected scope variant: {} for path {}",
        func, found, path
    );
    ConstructorError {
        error_type: ConstructorErrorType::CompilerBug(format!(
            "{}: unexpected scope '{}' for path '{}'",
            func, found, path
        )),
        position: Range::zero(),
    }
}
