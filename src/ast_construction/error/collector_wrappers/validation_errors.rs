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
    error::{EK, ErrorCollector, Phase, Pl, Sv},
};

impl ErrorCollector {
    /// Wrapper function for InvalidParamNumbersForInfix error.
    pub fn invalid_param_numbers_for_infix(
        &mut self,
        range: Range,
        phase: Phase,
        got_params: usize,
    ) {
        self.emit(
            EK::InvalidParamNumbersForInfix,
            range,
            phase,
            Sv::Error,
            Pl::Num(got_params),
        );
    }

    /// Wrapper function for InvalidParamNumbersForPrefix error.
    pub fn invalid_param_numbers_for_prefix(
        &mut self,
        range: Range,
        phase: Phase,
        got_params: usize,
    ) {
        self.emit(
            EK::InvalidParamNumbersForPrefix,
            range,
            phase,
            Sv::Error,
            Pl::Num(got_params),
        );
    }
}
