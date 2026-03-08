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

    pub fn infix_or_postfix_op_not_found(&mut self, range: Range, symbol: &str) {
        self.emit(
            EK::InfixOrPostfixOpNotFound,
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

    pub fn no_return_func_in_expr(&mut self, range: Range, func_name: &str) {
        self.emit(
            EK::NoReturnFuncInExpr,
            range,
            Ph::GlobalDeclCollection,
            Sv::Error,
            Pl::Str(func_name.to_string()),
        );
    }

    pub fn member_access_on_primitive(&mut self, range: Range) {
        self.emit(
            EK::MemberAccessOnPrimitive,
            range,
            Ph::GlobalDeclCollection,
            Sv::Error,
            Pl::None,
        );
    }

    pub fn member_field_not_found(
        &mut self,
        range: Range,
        struct_name: String,
        field_name: String,
    ) {
        self.emit(
            EK::MemberFieldNotFound,
            range,
            Ph::GlobalDeclCollection,
            Sv::Error,
            Pl::StrPair(struct_name, field_name),
        );
    }

    pub fn member_func_not_found(&mut self, range: Range, struct_name: String, func_name: String) {
        self.emit(
            EK::MemberFuncNotFound,
            range,
            Ph::GlobalDeclCollection,
            Sv::Error,
            Pl::StrPair(struct_name, func_name),
        );
    }

    pub fn arg_order_incorrect(&mut self, range: Range, func_name: &str, param_label: &str) {
        self.emit(
            EK::ArgOrderIncorrect,
            range,
            Ph::GlobalDeclCollection,
            Sv::Error,
            Pl::StrPair(func_name.to_string(), param_label.to_string()),
        );
    }

    pub fn duplicate_arg(&mut self, range: Range, func_name: &str, param_label: &str) {
        self.emit(
            EK::DuplicateArg,
            range,
            Ph::GlobalDeclCollection,
            Sv::Error,
            Pl::StrPair(func_name.to_string(), param_label.to_string()),
        );
    }

    pub fn extra_arg(&mut self, range: Range, func_name: &str) {
        self.emit(
            EK::ExtraArg,
            range,
            Ph::GlobalDeclCollection,
            Sv::Error,
            Pl::Str(func_name.to_string()),
        );
    }

    pub fn missing_arg(&mut self, range: Range, func_name: &str) {
        self.emit(
            EK::MissingArg,
            range,
            Ph::GlobalDeclCollection,
            Sv::Error,
            Pl::Str(func_name.to_string()),
        );
    }

    pub fn missing_arg_label(&mut self, range: Range, func_name: &str) {
        self.emit(
            EK::MissingArgLabel,
            range,
            Ph::GlobalDeclCollection,
            Sv::Error,
            Pl::Str(func_name.to_string()),
        );
    }

    pub fn type_annotation_mismatch(
        &mut self,
        range: Range,
        annotation_type: String,
        expr_type: String,
    ) {
        self.emit(
            EK::TypeAnnotationMismatch,
            range,
            Ph::GlobalDeclCollection,
            Sv::Error,
            Pl::StrPair(annotation_type.to_string(), expr_type.to_string()),
        );
    }

    pub fn invalid_struct_stmt(&mut self, range: Range, stmt_kind: String) {
        self.emit(
            EK::InvalidStructStmt,
            range,
            Ph::GlobalDeclCollection,
            Sv::Error,
            Pl::Str(stmt_kind),
        );
    }

    pub fn type_not_found(&mut self, range: Range, type_name: String) {
        self.emit(
            EK::TypeNotFound,
            range,
            Ph::GlobalDeclCollection,
            Sv::Error,
            Pl::Str(type_name),
        );
    }

    pub fn no_type_annotation_or_def_val(&mut self, range: Range) {
        self.emit(
            EK::NoTypeAnnotationOrDefVal,
            range,
            Ph::GlobalDeclCollection,
            Sv::Error,
            Pl::None,
        );
    }

    pub fn global_func_cannot_be_static(&mut self, range: Range, func_name: &str) {
        self.emit(
            EK::GlobalFuncCannotBeStatic,
            range,
            Ph::GlobalDeclCollection,
            Sv::Error,
            Pl::Str(func_name.to_string()),
        );
    }

    pub fn wrong_param_count_for_infix(
        &mut self,
        range: Range,
        op_symbol: &str,
        param_count: usize,
    ) {
        self.emit(
            EK::WrongParamCountForInfix,
            range,
            Ph::GlobalDeclCollection,
            Sv::Error,
            Pl::StrAndNum(op_symbol.to_string(), param_count),
        );
    }

    pub fn wrong_param_count_for_prefix(
        &mut self,
        range: Range,
        op_symbol: &str,
        param_count: usize,
    ) {
        self.emit(
            EK::WrongParamCountForPrefix,
            range,
            Ph::GlobalDeclCollection,
            Sv::Error,
            Pl::StrAndNum(op_symbol.to_string(), param_count),
        );
    }

    pub fn wrong_param_count_for_postfix(
        &mut self,
        range: Range,
        op_symbol: &str,
        param_count: usize,
    ) {
        self.emit(
            EK::WrongParamCountForPostfix,
            range,
            Ph::GlobalDeclCollection,
            Sv::Error,
            Pl::StrAndNum(op_symbol.to_string(), param_count),
        );
    }
}
