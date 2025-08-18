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
    ExprToken, FuncCallArg, FuncParam, InfixAttrValue, InputAttribute, LiteralBind,
    ProtocolRequirement, StateVar, Statement,
};
use std::collections::HashMap;

peg::parser!(pub grammar kash_parser() for str {
    pub rule parse() -> Vec<Statement>
        = statements()

    // --- STATEMENTS ---

    rule statements() -> Vec<Statement>
        = __? statements:(statement() ** ((_? "\n" _?)+)) __? { statements }

    rule statement() -> Statement
        = func_decl_statement()
        / return_statement()
        / input_statement()
        / output_statement()
        / var_statement()
        / state_statement()
        / assign_statement()
        / func_call_statement()
        / if_statement()
        / if_else_statement()
        / struct_decl_statement()
        / protocol_decl_statement()
        / init_statement()
        / infix_statement()
        / prefix_statement()
        / postfix_statement()
        / block_statement()
        / expected!("statement")

    rule func_decl_statement() -> Statement
        = required_by:(r:identifier() _ { r })?
        "func" _ name:identifier() _? "(" _? params:(func_param() ** comma()) comma()? ")" _?
        return_type:("->" _? t:identifier() { t })? __? "{"
        __? body:statements() __?
        "}" {
            Statement::FuncDecl { required_by, name, params, return_type, body }
        }

    rule return_statement() -> Statement
        = "return" value:(_ v:expression() { v })? {
            Statement::Return { value }
        }

    rule input_statement() -> Statement
        = "input" _ name:identifier() value_type:(_? ":" _? t:identifier() { t })? def_val:(_? "=" _? v:expression() { v })? attrs:(_? a:input_attr() { a })* {
            Statement::Input { name, value_type, def_val, attrs }
        }

    rule output_statement() -> Statement
        = "output" _ name:identifier() _? ":" _? value_type:identifier() {
            Statement::Output { name, value_type }
        }

    rule var_statement() -> Statement
        = "var" _ name:identifier() value_type:(_? ":" _? t:identifier() { t })? _? "=" _? def_val:expression() {
            Statement::Var { name, value_type, def_val }
        }

    rule state_statement() -> Statement
        = "state" _? "{" __? vars:(state_var() ** ((_? "\n" _?)+)) __? "}" {
            Statement::State { vars }
        }

    rule assign_statement() -> Statement
        = target:id_chain() _ "=" _ value:expression() {
            Statement::Assign { target, value }
        }

    rule func_call_statement() -> Statement
        = name:id_chain() _? "(" __? args:func_call_args() ")" {
            Statement::FuncCall { name, args }
        }

    rule if_statement() -> Statement
        = "if" _ condition:expression() __ "{"
        __? body:statements() __?
        "}" {
            Statement::If { condition, body }
        }

    rule if_else_statement() -> Statement
        = "if" _ condition:expression() __ "{"
        __? body:statements() __?
        "}" __ "else" __ "{"
        __? else_body:statements() __?
        "}" {
            Statement::IfElse { condition, body, else_body }
        }

    rule struct_decl_statement() -> Statement
        = "struct" _ name:identifier() inherits:(_? ":" _? i:(identifier() ** comma()) comma()? { i })? _? "{"
        __? body:statements() __?
        "}" {
            Statement::StructDecl {
                name,
                inherits: match inherits {
                    Some(inherits) => inherits,
                    None => Vec::new()
                },
                body
            }
        }

    rule protocol_decl_statement() -> Statement
        = "protocol" _ name:identifier() inherits:(_? ":" _? i:(identifier() ** comma()) comma()? { i })? _? "{"
        __? requires:(protocol_requirement() ** "\n") __?
        "}" {
            Statement::ProtocolDecl { name, inherits: match inherits {
                Some(inherits) => inherits,
                None => Vec::new()
            }, requires }
        }

    rule init_statement() -> Statement
        = literal_bind:(l:literal_bind() _ { l })? "init" _? "(" _? params:(func_param() ** comma()) comma()? ")" __? "{"
        __? body:statements() __?
        "}" {
            Statement::Init { literal_bind, params, body }
        }

    rule infix_statement() -> Statement
        = "infix" _ symbol:operator() _? "(" _? params:(func_param() ** comma()) comma()? ")" _? "->" _? return_type:identifier() __? "{"
        __? attrs:infix_attrs() __?
        "}" __? ":" __? "{"
        __? body:statements() __?
        "}" {
            Statement::Infix { symbol, params, return_type, attrs, body }
        }

    rule prefix_statement() -> Statement
        = "prefix" _ symbol:operator() _? "(" _? params:(func_param() ** comma()) comma()? ")" _? "->" _? return_type:identifier() __? "{"
        __? body:statements() __?
        "}" {
            Statement::Prefix { symbol, params, return_type, body }
        }

    rule postfix_statement() -> Statement
        = "postfix" _ symbol:operator() _? "(" _? params:(func_param() ** comma()) comma()? ")" _? "->" _? return_type:identifier() __? "{"
        __? body:statements() __?
        "}" {
            Statement::Postfix { symbol, params, return_type, body }
        }

    rule block_statement() -> Statement
        = "{" _? statements:statements() _? "}" {
            Statement::Block { statements }
        }

    // --- STATEMENT HELPERS ---

    // Function Parameter
    rule func_param() -> FuncParam
        = label:param_label()? name:identifier() value_type:(_? ":" _? t:identifier() { t })? def_val:(_? "=" _? v:expression() { v })? {
            FuncParam { label, name, value_type, def_val }
        }

    rule param_label() -> String
        = label:identifier() _ { label }

    // Input Attribute
    rule input_attr() -> InputAttribute
        = "#" name:identifier() opt_args:("(" _? args:(expression() ** comma()) comma()? ")" { args })? {
            InputAttribute { name, args: match opt_args {
                None => vec![],
                Some(args) => args
            } }
        }

    // State Variable
    rule state_var() -> StateVar
        = name:identifier() value_type:(_? ":" _? t:identifier() { t })? _? "=" _? def_val:expression() {
            StateVar { name, value_type, def_val }
        }

    // Function Call Argument
    rule func_call_args() -> Vec<FuncCallArg>
        = entries:((label:(n:identifier() _? ":" _? { n })? value:expression() {
            FuncCallArg { label, value }
        }) ** comma()) comma()? {
            entries
        }

    // --- Protocol Requirements ---

    rule protocol_requirement() -> ProtocolRequirement
        = protocol_func()
        / protocol_infix()
        / protocol_prefix()
        / protocol_postfix()

    rule protocol_func() -> ProtocolRequirement
        = "func" _ name:identifier() _? "(" _? params:(func_param() ** comma()) comma()? ")" _? return_type:("->" _? t:identifier() { t })? {
            ProtocolRequirement::Func { name, params, return_type }
        }

    rule protocol_infix() -> ProtocolRequirement
        = "infix" _ symbol:operator() _? "(" _? params:(func_param() ** comma()) comma()? ")" _? "->" _? return_type:identifier()
        attrs:("{" a:infix_attrs() "}" { a })? {
            ProtocolRequirement::Infix { symbol, params, return_type, attrs }
        }

    rule protocol_prefix() -> ProtocolRequirement
        = "prefix" _ symbol:operator() _? "(" _? params:(func_param() ** comma()) comma()? ")" _? "->" _? return_type:identifier() {
            ProtocolRequirement::Prefix { symbol, params, return_type }
        }

    rule protocol_postfix() -> ProtocolRequirement
        = "postfix" _ symbol:operator() _? "(" _? params:(func_param() ** comma()) comma()? ")" _? "->" _? return_type:identifier() {
            ProtocolRequirement::Postfix { symbol, params, return_type }
        }

    // Literal Binding
    rule literal_bind() -> LiteralBind
        = "intliteral" { LiteralBind::IntLiteral }
        / "floatliteral" { LiteralBind::FloatLiteral }
        / "boolliteral" { LiteralBind::BoolLiteral }

    // Infix Attributes
    rule infix_attrs() -> HashMap<String, InfixAttrValue>
        = entries:((key:identifier() _? ":" _? value:(
            v:identifier() { InfixAttrValue::String(v) }
            / v:integer() { InfixAttrValue::Integer(v)}
        ) {
            (key, value)
        }) ** comma()) comma()? {
            HashMap::from_iter(entries)
        }

    // --- EXPRESSIONS ---

    pub rule expression() -> Vec<ExprToken>
        = (
            !(__? "\n" / __? ")" / __? "}")
            first:expr_token()?
            rest:(
                ops:(
                    __? op:operator() {
                        ExprToken::Operator(op)
                    }
                )+
                __? token:expr_token() {
                    (ops, token)
                }
            )*
            last:operator()? {
                let mut tokens = match first {
                    Some(first) => vec![first],
                    None => vec![],
                };
                for (ops, token) in rest {
                    tokens.extend(ops);
                    tokens.push(token);
                }
                if let Some(op) = last { tokens.push(ExprToken::Operator(op)); }

                tokens
            }
        )
        / "(" _ expr:expression() _ ")" { expr }
        / expected!("expression")

    rule expr_token() -> ExprToken
        = token:(
            literal()
            / func_call()
            / (ids:id_chain() { ExprToken::Identifier(ids) })
        ) {
            token
        }

    rule func_call() -> ExprToken
        = name:id_chain() _? "(" __? args:func_call_args() ")" {
            ExprToken::FuncCall { name, args }
        }

    rule literal() -> ExprToken
        = decimal:decimal() { ExprToken::FloatLiteral(decimal) }
        / integer:integer() { ExprToken::IntLiteral(integer) }
        / boolean:boolean() { ExprToken::BoolLiteral(boolean)}

    // --- TOKENS ---

    rule identifier() -> String
        = quiet!{
            !reserved()
            n:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) { n.to_owned() }
        }
        / expected!("identifier")

    rule id_chain() -> Vec<String>
        = parent:identifier() children:(dot() c:id_chain() { c })? { match children {
            Some(children) => vec![parent].into_iter().chain(children.into_iter()).collect(),
            None => vec![parent]
        } }

    rule operator() -> String
        = quiet!{ op:$(['+' | '-' | '*' | '/' | '%' | '^' | '<' | '>' | '=' | '!' | '?' | '%' | '|' | '&']+) { op.to_owned() } }
        / expected!("operator")

    rule integer() -> u32
        = n:$(['0'..='9']+) { n.parse().unwrap() }

    rule decimal() -> f32
        = n:$(['0'..='9']+) "." d:$(['0'..='9']+) {
            (n.to_owned() + "." + d).parse().unwrap()
        }

    rule boolean() -> bool
        = quiet!{ "true" { true } / "false" { false } }
        / expected!("boolean")

    rule reserved()
        = ("input" / "output" / "var" / "state" / "func" / "return"
        / "if" / "else" / "struct" / "init" / "protocol" / "intliteral"
        / "floatliteral" / "boolliteral" / "infix" / "prefix"
        / "postfix") !['a'..='z' | 'A'..='Z' | '0'..='9' | '_']

    rule comment() = "//" (!['\n'] [_])* &['\n']

    rule comma() = __? "," __?

    rule dot() = __? "." __?

    rule _() = quiet!{([' ' | '\t'] / comment())+}

    rule __() = quiet!{([' ' | '\t' | '\n'] / comment())+}
});
