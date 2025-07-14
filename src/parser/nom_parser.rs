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

use crate::{Program, statement::statement};
use nom::{
    IResult, Parser,
    character::complete::line_ending,
    combinator::map,
    multi::{many1, separated_list0},
};

pub struct ASParser {}

impl ASParser {
    pub fn new() -> Self {
        ASParser {}
    }

    pub fn parse(&self, input: &str) -> Result<Program, String> {
        match program(input) {
            Ok((_, program)) => Ok(program),
            Err(err) => Err(format!("Parsing error: {:?}", err)),
        }
    }
}

pub fn program(s: &str) -> IResult<&str, Program> {
    map(
        separated_list0(many1(line_ending), statement),
        |statements| Program { statements },
    )
    .parse(s)
}
