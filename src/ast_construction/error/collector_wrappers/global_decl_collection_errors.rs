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

use crate::{
    Range,
    error::{EK, ErrorCollector, Ph, Pl, Sv},
};

impl ErrorCollector {
    pub fn top_level_struct_field(&mut self, range: Range, field_name: &str) {
        self.emit(
            EK::TopLevelStructField,
            range,
            Ph::GlobalDeclCollection,
            Sv::Error,
            Pl::Str(field_name.to_string()),
        );
    }

    pub fn var_not_found(&mut self, range: Range, var_name: &str) {
        self.emit(
            EK::VarNotFound,
            range,
            Ph::GlobalDeclCollection,
            Sv::Error,
            Pl::Str(var_name.to_string()),
        );
    }

    pub fn func_not_found(&mut self, range: Range, func_name: &str) {
        self.emit(
            EK::FuncNotFound,
            range,
            Ph::GlobalDeclCollection,
            Sv::Error,
            Pl::Str(func_name.to_string()),
        );
    }

    pub fn prefix_op_not_found(&mut self, range: Range, symbol: &str) {
        self.emit(
            EK::PrefixOpNotFound,
            range,
            Ph::GlobalDeclCollection,
            Sv::Error,
            Pl::Str(symbol.to_string()),
        );
    }

    pub fn op_not_associative(&mut self, range: Range, symbol: &str) {
        self.emit(
            EK::OpNotAssociative,
            range,
            Ph::GlobalDeclCollection,
            Sv::Error,
            Pl::Str(symbol.to_string()),
        );
    }
}
