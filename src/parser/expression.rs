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
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    bytes::complete::take_while,
    combinator::{map, opt},
    multi::separated_list0,
    number::complete::float,
};

pub fn number(s: &str) -> IResult<&str, Expression> {
    map(float, |num: f32| Expression::Literal(num)).parse(s)
}

pub fn operator(s: &str) -> IResult<&str, Operator> {
    alt((
        map(nom::bytes::complete::tag("+"), |_| Operator::Add),
        map(nom::bytes::complete::tag("-"), |_| Operator::Subtract),
        map(nom::bytes::complete::tag("*"), |_| Operator::Multiply),
        map(nom::bytes::complete::tag("/"), |_| Operator::Divide),
        map(nom::bytes::complete::tag("%"), |_| Operator::Modulo),
    ))
    .parse(s)
}

pub fn operator_expression(s: &str) -> IResult<&str, Expression> {
    map((number, operator, number), |(left, op, right)| {
        Expression::BinaryOp {
            op: op,
            left: Box::new(left),
            right: Box::new(right),
        }
    })
    .parse(s)
}

pub fn function_call(s: &str) -> IResult<&str, Expression> {
    let call_parser = (
        take_while(|c: char| c.is_alphanumeric() || c == '_'),
        tag("("),
        opt(separated_list0(tag(","), number)),
        tag(")"),
    );
    map(call_parser, |(name, _, args, _)| Expression::FunctionCall {
        name: name.to_string(),
        arguments: args.unwrap_or_default(),
    })
    .parse(s)
}

pub fn expression(s: &str) -> IResult<&str, Expression> {
    alt((number, operator_expression, function_call)).parse(s)
}
