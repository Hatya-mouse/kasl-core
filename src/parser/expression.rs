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

use crate::{Expression, Operator};
use nom::{
    AsChar, IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_till1},
    character::complete::{alphanumeric1, space0},
    combinator::{map, opt},
    multi::{many1, separated_list0},
    number::complete::float,
};

pub fn number(s: &str) -> IResult<&str, Expression> {
    map(float, |num: f32| Expression::Literal(num)).parse(s)
}

pub fn expression(s: &str) -> IResult<&str, Expression> {
    alt((
        map(
            (expr_term, space0, tag("+"), space0, expression),
            |(left, _, _, _, right)| Expression::BinaryOp {
                op: Operator::Add,
                left: Box::new(left),
                right: Box::new(right),
            },
        ),
        map(
            (expr_term, space0, tag("-"), space0, expression),
            |(left, _, _, _, right)| Expression::BinaryOp {
                op: Operator::Subtract,
                left: Box::new(left),
                right: Box::new(right),
            },
        ),
        expr_term,
    ))
    .parse(s)
}

pub fn expr_term(s: &str) -> IResult<&str, Expression> {
    let res = alt((
        map(
            (term_muldiv, space0, expr_factor),
            |((left, op), _, right)| Expression::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            },
        ),
        expr_factor,
    ))
    .parse(s);
    res
}

pub fn term_muldiv(s: &str) -> IResult<&str, (Expression, Operator)> {
    let res = alt((
        map(
            (
                take_till1(|c: char| c == '*' || c.is_newline()),
                space0,
                tag::<&str, &str, nom::error::Error<&str>>("*"),
                space0,
            ),
            |(left, _, _, _)| (left, Operator::Multiply),
        ),
        map(
            (
                take_till1(|c: char| c == '/' || c.is_newline()),
                space0,
                tag("/"),
                space0,
            ),
            |(left, _, _, _)| (left, Operator::Divide),
        ),
    ))
    .parse(s);

    if let Ok((remaining, (left, op))) = res {
        if let Ok((_, left_expr)) = expr_term(left) {
            Ok((remaining, (left_expr, op)))
        } else {
            Err(nom::Err::Error(nom::error::make_error(
                s,
                nom::error::ErrorKind::Tag,
            )))
        }
    } else {
        Err(nom::Err::Error(nom::error::make_error(
            s,
            nom::error::ErrorKind::Tag,
        )))
    }
}

pub fn expr_factor(s: &str) -> IResult<&str, Expression> {
    let res = map(
        (
            alt((
                number,
                identifier,
                map((tag("("), expression, tag(")")), |(_, expr, _)| expr),
            )),
            space0,
        ),
        |(expr, _)| expr,
    )
    .parse(s);
    res
}

pub fn identifier(s: &str) -> IResult<&str, Expression> {
    println!("Parsing identifier from: {}", s);
    let res = map(
        (
            symbol_name,
            opt(map(
                (
                    tag("("),
                    separated_list0((space0, tag(","), space0), expression),
                    tag(")"),
                ),
                |(_, args, _)| args,
            )),
        ),
        |(name, args)| match args {
            Some(args) => Expression::FunctionCall {
                name,
                arguments: args,
            },
            None => Expression::Identifier(name),
        },
    )
    .parse(s);
    println!("Parsed identifier: {:?}", res);
    res
}

pub fn symbol_name(s: &str) -> IResult<&str, String> {
    map(many1(alt((alphanumeric1, tag("_")))), |name| name.concat()).parse(s)
}
