#[cfg(test)]
mod tests {
    use crate::*;

    fn parse_and_interpret(source: &str) -> Result<Value, String> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().map_err(|e| format!("Lexer error: {}", e))?;
        
        let mut parser = Parser::new(tokens);
        let program = parser.parse().map_err(|e| format!("Parse error: {:?}", e))?;
        
        let mut interpreter = Interpreter::new();
        interpreter.interpret(&program).map_err(|e| format!("Runtime error: {:?}", e))
    }

    #[test]
    fn test_arithmetic() {
        assert_eq!(parse_and_interpret("2 + 3").unwrap(), Value::Number(5.0));
        assert_eq!(parse_and_interpret("10 - 4").unwrap(), Value::Number(6.0));
        assert_eq!(parse_and_interpret("3 * 4").unwrap(), Value::Number(12.0));
        assert_eq!(parse_and_interpret("15 / 3").unwrap(), Value::Number(5.0));
    }

    #[test]
    fn test_comparison() {
        assert_eq!(parse_and_interpret("5 > 3").unwrap(), Value::Boolean(true));
        assert_eq!(parse_and_interpret("2 < 1").unwrap(), Value::Boolean(false));
        assert_eq!(parse_and_interpret("4 == 4").unwrap(), Value::Boolean(true));
        assert_eq!(parse_and_interpret("5 != 3").unwrap(), Value::Boolean(true));
    }

    #[test]
    fn test_logical() {
        assert_eq!(parse_and_interpret("true and false").unwrap(), Value::Boolean(false));
        assert_eq!(parse_and_interpret("true or false").unwrap(), Value::Boolean(true));
        assert_eq!(parse_and_interpret("!true").unwrap(), Value::Boolean(false));
    }

    #[test]
    fn test_variables() {
        let source = r#"
            let x = 42;
            x
        "#;
        assert_eq!(parse_and_interpret(source).unwrap(), Value::Number(42.0));
    }

    #[test]
    fn test_functions() {
        let source = r#"
            fn add(a, b) {
                a + b
            }
            add(3, 4)
        "#;
        assert_eq!(parse_and_interpret(source).unwrap(), Value::Number(7.0));
    }

    #[test]
    fn test_if_statement() {
        let source = r#"
            if (5 > 3) {
                "yes"
            } else {
                "no"
            }
        "#;
        assert_eq!(parse_and_interpret(source).unwrap(), Value::String("yes".to_string()));
    }

    #[test]
    fn test_string_concatenation() {
        let source = r#"
            "Hello " + "World"
        "#;
        assert_eq!(parse_and_interpret(source).unwrap(), Value::String("Hello World".to_string()));
    }

    #[test]
    fn test_nested_functions() {
        let source = r#"
            fn outer(x) {
                fn inner(y) {
                    x + y
                }
                inner(5)
            }
            outer(10)
        "#;
        assert_eq!(parse_and_interpret(source).unwrap(), Value::Number(15.0));
    }

    #[test]
    fn test_recursive_function() {
        let source = r#"
            fn factorial(n) {
                if (n <= 1) {
                    1
                } else {
                    n * factorial(n - 1)
                }
            }
            factorial(5)
        "#;
        assert_eq!(parse_and_interpret(source).unwrap(), Value::Number(120.0));
    }
    
    #[test]
    fn test_arrays() {
        let source = r#"[1, 2, 3]"#;
        assert_eq!(parse_and_interpret(source).unwrap(), Value::Array(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0)
        ]));
    }
    
    #[test]
    fn test_array_indexing() {
        let source = r#"
            let arr = ["a", "b", "c"];
            arr[1]
        "#;
        assert_eq!(parse_and_interpret(source).unwrap(), Value::String("b".to_string()));
    }
    
    #[test]
    fn test_array_length() {
        let source = r#"
            let arr = [1, 2, 3, 4, 5];
            len(arr)
        "#;
        assert_eq!(parse_and_interpret(source).unwrap(), Value::Number(5.0));
    }
    
    #[test]
    fn test_string_length() {
        let source = r#"len("hello")"#;
        assert_eq!(parse_and_interpret(source).unwrap(), Value::Number(5.0));
    }
    
    #[test]
    fn test_while_loop() {
        let source = r#"
            let sum = 0;
            let i = 1;
            while (i <= 5) {
                sum = sum + i;
                i = i + 1;
            }
            sum
        "#;
        assert_eq!(parse_and_interpret(source).unwrap(), Value::Number(15.0));
    }
    
    #[test]
    fn test_for_loop_array() {
        let source = r#"
            let sum = 0;
            for num in [1, 2, 3, 4, 5] {
                sum = sum + num;
            }
            sum
        "#;
        assert_eq!(parse_and_interpret(source).unwrap(), Value::Number(15.0));
    }
    
    #[test]
    fn test_for_loop_string() {
        let source = r#"
            let result = "";
            for char in "abc" {
                result = result + char;
            }
            result
        "#;
        assert_eq!(parse_and_interpret(source).unwrap(), Value::String("abc".to_string()));
    }
    
    #[test]
    fn test_type_function() {
        let source = r#"type(42)"#;
        assert_eq!(parse_and_interpret(source).unwrap(), Value::String("number".to_string()));
    }
    
    #[test]
    fn test_str_function() {
        let source = r#"str(123)"#;
        assert_eq!(parse_and_interpret(source).unwrap(), Value::String("123".to_string()));
    }
    
    #[test]
    fn test_num_function() {
        let source = r#"num("42.5")"#;
        assert_eq!(parse_and_interpret(source).unwrap(), Value::Number(42.5));
    }
    
    #[test]
    fn test_push_function() {
        let source = r#"
            let arr = [1, 2];
            push(arr, 3)
        "#;
        assert_eq!(parse_and_interpret(source).unwrap(), Value::Array(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0)
        ]));
    }
    
    #[test]
    fn test_pop_function() {
        let source = r#"
            let arr = [1, 2, 3];
            pop(arr)
        "#;
        assert_eq!(parse_and_interpret(source).unwrap(), Value::Number(3.0));
    }
    
    #[test]
    fn test_nested_arrays() {
        let source = r#"
            let matrix = [[1, 2], [3, 4]];
            matrix[1][0]
        "#;
        assert_eq!(parse_and_interpret(source).unwrap(), Value::Number(3.0));
    }
}