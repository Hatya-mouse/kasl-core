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

use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorVariant {
    UndefinedSymbol(String),
    UndefinedFunction(String),
    SymbolAlreadyDefined(String),
}

impl Display for ErrorVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorVariant::UndefinedSymbol(name) => write!(f, "Undefined symbol: {}", name),
            ErrorVariant::UndefinedFunction(name) => write!(f, "Undefined function: {}", name),
            ErrorVariant::SymbolAlreadyDefined(name) => {
                write!(f, "Symbol already defined: {}", name)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticError {
    pub errors: Vec<ErrorVariant>,
}

impl SemanticError {
    pub fn new() -> Self {
        SemanticError { errors: Vec::new() }
    }
}

impl Error for SemanticError {}

impl Display for SemanticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for error in &self.errors {
            writeln!(f, "{}", error)?;
        }
        Ok(())
    }
}
