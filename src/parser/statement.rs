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

use crate::{
    AssignmentStatement, InputDeclarationStatement, OutputDeclarationStatement, Statement,
    VariableDeclarationStatement,
    ast::{ForLoopStatement, InputAttribute},
    expression::{expression, symbol_name},
};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{space0, space1},
    combinator::{map, opt},
    multi::{many0, separated_list0},
};

pub fn input_attr(s: &str) -> IResult<&str, InputAttribute> {
    map(
        (
            tag("#"),
            symbol_name,
            opt((tag("("), separated_list0(tag(","), expression), tag(")"))),
        ),
        |(_, name, value)| InputAttribute {
            name: name.to_string(),
            value: value.iter().flat_map(|(_, vals, _)| vals.clone()).collect(),
        },
    )
    .parse(s)
}

pub fn input_decl(s: &str) -> IResult<&str, Statement> {
    map(
        (
            tag("input"),
            space1,
            symbol_name,
            many0((space1, input_attr)),
        ),
        |(_, _, name, input_attrs)| {
            Statement::InputDeclaration(InputDeclarationStatement {
                name: name.to_string(),
                input_attrs: input_attrs.iter().map(|(_, attr)| attr.clone()).collect(),
                line: 0,
            })
        },
    )
    .parse(s)
}

pub fn output_decl(s: &str) -> IResult<&str, Statement> {
    map(
        (tag("output"), space1, symbol_name),
        |(_, _, name): (_, _, String)| {
            Statement::OutputDeclaration(OutputDeclarationStatement { name, line: 0 })
        },
    )
    .parse(s)
}

pub fn variable_decl(s: &str) -> IResult<&str, Statement> {
    map(
        (
            tag("var"),
            space1,
            symbol_name,
            space0,
            tag("="),
            space0,
            expression,
        ),
        |(_, _, name, _, _, _, value)| {
            Statement::VariableDeclaration(VariableDeclarationStatement {
                name: name.to_string(),
                initial_value: value,
                line: 0,
            })
        },
    )
    .parse(s)
}

pub fn assignment(s: &str) -> IResult<&str, Statement> {
    map(
        (symbol_name, space0, tag("="), space0, expression),
        |(target_name, _, _, _, value)| {
            Statement::Assignment(AssignmentStatement {
                target_name: target_name.to_string(),
                value,
                line: 0,
            })
        },
    )
    .parse(s)
}

pub fn for_loop(s: &str) -> IResult<&str, Statement> {
    map(
        (
            tag("for"),
            space1,
            symbol_name,
            space1,
            tag("in"),
            space1,
            expression,
            space1,
            tag("{"),
            many0(statement),
            tag("}"),
        ),
        |(_, _, variable_name, _, _, _, iterable, _, _, body, _)| {
            Statement::ForLoop(ForLoopStatement {
                variable_name: variable_name.to_string(),
                iterable,
                body,
                line: 0,
            })
        },
    )
    .parse(s)
}

pub fn statement(s: &str) -> IResult<&str, Statement> {
    map(
        (
            space0,
            alt((input_decl, output_decl, variable_decl, assignment, for_loop)),
            space0,
        ),
        |(_, stmt, _)| stmt,
    )
    .parse(s)
}
