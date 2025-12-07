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

pub trait Scope<'a> {
    /// Get a mutable reference to a Function by name. Returns None if the Function is not found.
    fn get_func_mut(&mut self, name: &str) -> Option<&mut Function<'a>>;

    /// Get a mutable reference to a TypeDef by name. Returns None if the TypeDef is not found.
    fn get_type_def_mut(&mut self, name: &str) -> Option<&mut TypeDef<'a>>;

    /// Get a mutable reference to a StateVar by name. Returns None if the StateVar is not found.
    fn get_state_mut(&mut self, name: &str) -> Option<&mut StateVar<'a>>;

    /// Get a mutable reference to an InputVar by name. Returns None if the InputVar is not found.
    fn get_input_mut(&mut self, name: &str) -> Option<&mut InputVar<'a>>;

    /// Get a mutable reference to an OutputVar by name. Returns None if the OutputVar is not found.
    fn get_output_mut(&mut self, name: &str) -> Option<&mut OutputVar<'a>>;

    /// Get a mutable reference to a Variable by name. Returns None if the Variable is not found.
    fn get_var_mut(&mut self, name: &str) -> Option<&mut Variable<'a>>;

    /// Get a mutable reference to an Operator by name. Returns None if the Operator is not found.
    fn get_operator_mut(&mut self, name: &str) -> Option<&mut Operator<'a>>;

    /// Get a mutable reference to a FuncParam by name. Returns None if the FuncParam is not found.
    fn get_func_param_mut(&mut self, name: &str) -> Option<&mut FuncParam<'a>>;
}
