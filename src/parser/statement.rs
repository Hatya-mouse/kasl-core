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
    expression::expression,
};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::space1,
    combinator::{map, opt},
    multi::{many0, separated_list0},
};

pub fn input_attr(s: &str) -> IResult<&str, InputAttribute> {
    map(
        (
            tag("#"),
            take_while(|c: char| c.is_alphanumeric() || c == '_'),
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
            take_while(|c: char| c.is_alphanumeric() || c == '_'),
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
        (
            tag("output"),
            space1,
            take_while(|c: char| c.is_alphanumeric() || c == '_'),
        ),
        |(_, _, name): (_, _, &str)| {
            Statement::OutputDeclaration(OutputDeclarationStatement {
                name: name.to_string(),
                line: 0,
            })
        },
    )
    .parse(s)
}

pub fn variable_decl(s: &str) -> IResult<&str, Statement> {
    map(
        (
            tag("var"),
            space1,
            take_while(|c: char| c.is_alphanumeric() || c == '_'),
            tag("="),
            expression,
        ),
        |(_, _, name, _, value)| {
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
        (
            take_while(|c: char| c.is_alphanumeric() || c == '_'),
            tag("="),
            expression,
        ),
        |(target_name, _, value)| {
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
            take_while(|c: char| c.is_alphanumeric() || c == '_'),
            tag("in"),
            expression,
            space1,
            tag("{"),
            many0(statement),
            tag("}"),
        ),
        |(_, _, variable_name, _, iterable, _, _, body, _)| {
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
    alt((input_decl, output_decl, variable_decl, assignment, for_loop)).parse(s)
}
