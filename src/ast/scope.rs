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

use crate::{FuncParam, Function, InputVar, Operator, OutputVar, StateVar, TypeDef, Variable};

pub trait Scope {
    /// Get an immutable reference to a Function by its name. Returns None if the Function is not found.
    fn get_func(&self, _name: &str) -> Option<&Function> {
        None
    }

    /// Get a mutable reference to a Function by its name. Returns None if the Function is not found.
    fn get_func_mut(&mut self, _name: &str) -> Option<&mut Function> {
        None
    }

    /// Get an immutable reference to a TypeDef by its name. Returns None if the TypeDef is not found.
    fn get_type_def(&self, _name: &str) -> Option<&TypeDef> {
        None
    }

    /// Get a mutable reference to a TypeDef by its name. Returns None if the TypeDef is not found.
    fn get_type_def_mut(&mut self, _name: &str) -> Option<&mut TypeDef> {
        None
    }

    /// Get an immutable reference to a StateVar by its name. Returns None if the StateVar is not found.
    fn get_state(&self, _name: &str) -> Option<&StateVar> {
        None
    }

    /// Get a mutable reference to a StateVar by its name. Returns None if the StateVar is not found.
    fn get_state_mut(&mut self, _name: &str) -> Option<&mut StateVar> {
        None
    }

    /// Get an immutable reference to an InputVar by its name. Returns None if the InputVar is not found.
    fn get_input(&self, _name: &str) -> Option<&InputVar> {
        None
    }

    /// Get a mutable reference to an InputVar by its name. Returns None if the InputVar is not found.
    fn get_input_mut(&mut self, _name: &str) -> Option<&mut InputVar> {
        None
    }

    /// Get an immutable reference to an OutputVar by its name. Returns None if the OutputVar is not found.
    fn get_output(&self, _name: &str) -> Option<&OutputVar> {
        None
    }

    /// Get a mutable reference to an OutputVar by its name. Returns None if the OutputVar is not found.
    fn get_output_mut(&mut self, _name: &str) -> Option<&mut OutputVar> {
        None
    }

    /// Get an immutable reference to a Variable by its name. Returns None if the Variable is not found.
    fn get_var(&self, _name: &str) -> Option<&Variable> {
        None
    }

    /// Get a mutable reference to a Variable by its name. Returns None if the Variable is not found.
    fn get_var_mut(&mut self, _name: &str) -> Option<&mut Variable> {
        None
    }

    /// Get an immutable reference to an Operator by its name. Returns None if the Operator is not found.
    fn get_operator(&self, _name: &str) -> Option<&Operator> {
        None
    }

    /// Get a mutable reference to an Operator by its name. Returns None if the Operator is not found.
    fn get_operator_mut(&mut self, _name: &str) -> Option<&mut Operator> {
        None
    }

    /// Get an immutable reference to a FuncParam by its name. Returns None if the FuncParam is not found.
    fn get_func_param(&self, _name: &str) -> Option<&FuncParam> {
        None
    }

    /// Get a mutable reference to a FuncParam by its name. Returns None if the FuncParam is not found.
    fn get_func_param_mut(&mut self, _name: &str) -> Option<&mut FuncParam> {
        None
    }
}
