//
//  Copyright 2025-2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

use kasl::error::{ErrorKind, ErrorRecord};

pub fn assert_error(error: &[ErrorRecord], expected: ErrorKind) {
    assert!(error.iter().any(|r| r.key.kind == expected))
}

#[macro_export]
macro_rules! assert_func_ctx_snapshot {
    ($func_ctx:expr) => {
        use insta::{assert_yaml_snapshot, sorted_redaction};
        assert_yaml_snapshot!($func_ctx, {
            ".funcs" => sorted_redaction(),
            ".member_functions" => sorted_redaction(),
            ".global_functions" => sorted_redaction()
        });
    };
}

#[macro_export]
macro_rules! assert_scope_registry_snapshot {
    ($scope_registry:expr) => {
        use insta::{assert_yaml_snapshot, sorted_redaction};
        assert_yaml_snapshot!($scope_registry, {
            ".scopes" => sorted_redaction(),
            ".variables" => sorted_redaction(),
            ".global_scope_ids" => sorted_redaction(),
            ".**.name_to_id" => sorted_redaction()
        });
    };
}

#[macro_export]
macro_rules! assert_type_registry_snapshot {
    ($type_registry:expr) => {
        use insta::{assert_yaml_snapshot, sorted_redaction};
        assert_yaml_snapshot!($type_registry, {
            ".structs" => sorted_redaction(),
            ".name_to_id" => sorted_redaction(),
            ".**.indices" => sorted_redaction(),
        });
    };
}
