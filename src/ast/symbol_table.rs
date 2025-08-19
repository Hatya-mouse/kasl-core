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

use crate::{FuncParam, TypeName};

#[derive(Debug, PartialEq, Clone)]
pub struct VariableInfo {
    pub name: String,
    pub ty: Option<TypeName>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionInfo {
    pub name: String,
    pub params: Vec<FuncParam>,
    pub return_ty: Option<TypeName>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructInfo {
    pub name: String,
    pub variables: Vec<VariableInfo>,
    pub functions: Vec<FunctionInfo>,
}

/// Wrapper enum for symbols in the resolver.
#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    Type(TypeName),
    Variable(VariableInfo),
    Function(FunctionInfo),
    Struct(StructInfo),
}

/// SymbolTable is a symbol table for the resolver.
#[derive(Debug, PartialEq, Clone)]
pub struct SymbolTable {
    pub parent: Option<Box<SymbolTable>>,
    pub types: Vec<TypeName>,
    pub inputs: Vec<VariableInfo>,
    pub outputs: Vec<VariableInfo>,
    pub states: Vec<VariableInfo>,
    pub variables: Vec<VariableInfo>,
    pub structs: Vec<StructInfo>,
    pub protocols: Vec<StructInfo>,
    pub functions: Vec<FunctionInfo>,
}

impl SymbolTable {
    pub fn resolve_ty(&self, name: &str) -> Option<Symbol> {
        let parent = self.parent.as_ref();
        let parent_ty = parent.and_then(|parent| parent.resolve_ty(name));
        let self_ty = self
            .types
            .iter()
            .find(|ty| ty.str() == name)
            .map(|ty| Symbol::Type(ty.clone()));

        parent_ty.or(self_ty)
    }

    pub fn resolve_var(&self, name: &str) -> Option<Symbol> {
        let parent = self.parent.as_ref();
        let parent_var = parent.and_then(|parent| parent.resolve_var(name));
        let input = self
            .inputs
            .iter()
            .find(|input| input.name == name)
            .map(|input| Symbol::Variable(input.clone()));
        let output = self
            .outputs
            .iter()
            .find(|output| output.name == name)
            .map(|output| Symbol::Variable(output.clone()));
        let state = self
            .states
            .iter()
            .find(|state| state.name == name)
            .map(|state| Symbol::Variable(state.clone()));
        let variable = self
            .variables
            .iter()
            .find(|var| var.name == name)
            .map(|var| Symbol::Variable(var.clone()));

        parent_var.or(input).or(output).or(state).or(variable)
    }

    pub fn resolve_struct(&self, name: &str) -> Option<Symbol> {
        let parent = self.parent.as_ref();
        let parent_struct = parent.and_then(|parent| parent.resolve_struct(name));
        let struct_symbol = self
            .structs
            .iter()
            .find(|struct_info| struct_info.name == name)
            .map(|struct_info| Symbol::Struct(struct_info.clone()));
        let protocol = self
            .protocols
            .iter()
            .find(|protocol| protocol.name == name)
            .map(|protocol| Symbol::Struct(protocol.clone()));

        parent_struct.or(struct_symbol).or(protocol)
    }

    pub fn resolve_func(&self, name: &str) -> Option<Symbol> {
        let parent = self.parent.as_ref();
        let parent_func = parent.and_then(|parent| parent.resolve_func(name));
        let func = self
            .functions
            .iter()
            .find(|func| func.name == name)
            .map(|func| Symbol::Function(func.clone()));

        parent_func.or(func)
    }
}
