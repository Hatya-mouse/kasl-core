#[cfg(test)]
mod tests {
    use crate::{
        AssignmentStatement, Expression, InputDeclarationStatement, Lexer,
        OutputDeclarationStatement, Parser, Statement, TokenType, Type,
    };

    #[test]
    fn test_basic_shader_tokenize() {
        let code = "input float in_buffer = 0.0
                    input float gain = 1.0
                    output float out_buffer
                    out_buffer = in_buffer * gain";
        let lexer = Lexer::new(code.to_string());
        let tokens = lexer.tokenize();

        println!("Tokenized: {:?}", tokens);

        assert!(!tokens.is_empty());
        assert_eq!(tokens[0], TokenType::Input);
        assert_eq!(tokens[2], TokenType::Identifier("in_buffer".into()));
        assert_eq!(tokens[3], TokenType::Assign);
        assert_eq!(tokens[4], TokenType::FloatLiteral(0.0));

        assert_eq!(tokens[10], TokenType::FloatLiteral(1.0));
    }

    #[test]
    fn test_parsing() {
        let code = "input float x = 1.0
                    output float y

                    y = pow(x, 2)";
        let lexer = Lexer::new(code.to_string());
        let tokens = lexer.tokenize();

        let parser = Parser::new(tokens);
        let program = parser.parse();

        println!("Parsed program: {:?}", program);

        assert!(program.is_ok());
        let program = program.unwrap();
        assert!(!program.statements.is_empty());

        assert_eq!(
            program.statements[0],
            Statement::InputDeclaration(InputDeclarationStatement {
                name: "x".to_string(),
                data_type: Type::Float,
                initial_value: Some(Expression::Literal(1.0)),
                range: None,
            })
        );
        assert_eq!(
            program.statements[1],
            Statement::OutputDeclaration(OutputDeclarationStatement {
                name: "y".to_string(),
                data_type: Type::Float,
            })
        );
        assert_eq!(
            program.statements[2],
            Statement::Assignment(AssignmentStatement {
                target_name: "y".to_string(),
                value: Expression::FunctionCall {
                    name: "pow".to_string(),
                    arguments: vec![
                        Expression::Identifier("x".to_string()),
                        Expression::Literal(2.0)
                    ]
                }
            })
        );
    }
}
