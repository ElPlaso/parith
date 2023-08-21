#[cfg(test)]
mod display_tests {
    use crate::expression::{BinaryOperator, Expression, UnaryOperator};

    #[test]
    fn test_display_integer() {
        let expr = Expression::Integer(42);
        assert_eq!(format!("{}", expr), "42");
    }

    #[test]
    fn test_display_variable() {
        let expr = Expression::Variable("x".to_string());
        assert_eq!(format!("{}", expr), "x");
    }

    #[test]
    fn test_display_boolean() {
        let expr1 = Expression::Boolean(true);
        assert_eq!(format!("{}", expr1), "T");

        let expr2 = Expression::Boolean(false);
        assert_eq!(format!("{}", expr2), "F");
    }

    #[test]
    fn test_display_binary_op() {
        let expr = Expression::BinaryOp {
            op: BinaryOperator::Add,
            lhs: Box::new(Expression::Integer(3)),
            rhs: Box::new(Expression::Integer(4)),
        };
        assert_eq!(format!("{}", expr), "3 + 4");
    }

    #[test]
    fn test_display_unary_op() {
        let expr = Expression::UnaryOp {
            op: UnaryOperator::Not,
            child: Box::new(Expression::Boolean(true)),
        };
        assert_eq!(format!("{}", expr), "!T");
    }

    #[test]
    fn test_display_func() {
        let expr = Expression::Func {
            param: "x".to_string(),
            body: Box::new(Expression::BinaryOp {
                op: BinaryOperator::Multiply,
                lhs: Box::new(Expression::Variable("x".to_string())),
                rhs: Box::new(Expression::Integer(2)),
            }),
        };
        assert_eq!(format!("{}", expr), "func x => x * 2");
    }

    #[test]
    fn test_display_if() {
        let expr = Expression::If {
            condition: Box::new(Expression::Boolean(true)),
            then_expr: Box::new(Expression::Integer(42)),
            else_expr: Box::new(Expression::Integer(0)),
        };
        assert_eq!(format!("{}", expr), "if T then 42 else 0");
    }

    #[test]
    fn test_display_apply() {
        let expr = Expression::Apply {
            func_expr: Box::new(Expression::Variable("f".to_string())),
            arg_expr: Box::new(Expression::Integer(10)),
        };
        assert_eq!(format!("{}", expr), "f (10)");
    }
}

#[cfg(test)]
mod lexing_tests {
    use crate::expression::BinaryOperator;
    use crate::parser::{lex, LexItem};

    #[test]
    fn lex_integer() {
        let input = "123";
        let result = lex(input);
        assert_eq!(result, Ok(vec![LexItem::Integer(123)]));
    }

    #[test]
    fn lex_variable() {
        let input = "abc";
        let result = lex(input);
        assert_eq!(result, Ok(vec![LexItem::Variable("abc".to_string())]));
    }

    #[test]
    fn lex_boolean_true() {
        let input = "T";
        let result = lex(input);
        assert_eq!(result, Ok(vec![LexItem::Boolean(true)]));
    }

    #[test]
    fn lex_boolean_false() {
        let input = "F";
        let result = lex(input);
        assert_eq!(result, Ok(vec![LexItem::Boolean(false)]));
    }

    #[test]
    fn lex_binary_operator() {
        let input = "+";
        let result = lex(input);
        assert_eq!(result, Ok(vec![LexItem::BinaryOp(BinaryOperator::Add)]));
    }

    #[test]
    fn lex_addition_expression() {
        let input = "+(1, 1)";
        let result = lex(input);
        assert_eq!(
            result,
            Ok(vec![
                LexItem::BinaryOp(BinaryOperator::Add),
                LexItem::OpenParen,
                LexItem::Integer(1),
                LexItem::Comma,
                LexItem::Integer(1),
                LexItem::CloseParen
            ])
        );
    }

    #[test]
    fn lex_subtraction_expression() {
        let input = "-(1, 1)";
        let result = lex(input);
        assert_eq!(
            result,
            Ok(vec![
                LexItem::BinaryOp(BinaryOperator::Subtract),
                LexItem::OpenParen,
                LexItem::Integer(1),
                LexItem::Comma,
                LexItem::Integer(1),
                LexItem::CloseParen
            ])
        );
    }
}

#[cfg(test)]
mod arith_tests {
    use crate::parser::Parser;

    #[test]
    fn parse_var() {
        let mut prog = Parser::new(&"x");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("x", format!("{}", e));
    }

    #[test]
    fn parse_int() {
        let mut prog = Parser::new(&"123");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("123", format!("{}", e));
    }

    #[test]
    fn parse_bool() {
        let mut prog = Parser::new(&"T");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("T", format!("{}", e));
    }

    #[test]
    fn parse_plus() {
        let mut prog = Parser::new(&"+(1, 1)");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("1 + 1", format!("{}", e));
    }

    #[test]
    fn parse_nested_plus() {
        let mut prog = Parser::new(&"+(1, +(1, 1))");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("1 + 1 + 1", format!("{}", e));
    }

    #[test]
    fn parse_minus() {
        let mut prog = Parser::new(&"-(1, 1)");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("1 - 1", format!("{}", e));
    }

    #[test]
    fn parse_mult() {
        let mut prog = Parser::new(&"*(1, 1)");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("1 * 1", format!("{}", e));
    }

    #[test]
    fn parse_div() {
        let mut prog = Parser::new(&"/(1, 1)");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("1 / 1", format!("{}", e));
    }

    #[test]
    fn parse_lt() {
        let mut prog = Parser::new(&"<(1, 1)");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("1 < 1", format!("{}", e));
    }

    #[test]
    fn parse_and() {
        let mut prog = Parser::new(&"&(T, T)");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("T & T", format!("{}", e));
    }

    #[test]
    fn parse_or() {
        let mut prog = Parser::new(&"|(T, T)");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("T | T", format!("{}", e));
    }

    #[test]
    fn parse_not() {
        let mut prog = Parser::new(&"!T");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("!T", format!("{}", e));
    }

    #[test]
    fn parse_eq() {
        let mut prog = Parser::new(&"=(1, 1)");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("1 = 1", format!("{}", e));
    }

    #[test]
    fn parse_func() {
        let mut prog = Parser::new(&"func x => T");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("func x => T", format!("{}", e));
    }

    #[test]
    fn parse_app() {
        let mut prog = Parser::new(&"apply(func x => x, 1)");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("func x => x (1)", format!("{}", e));
    }

    #[test]
    fn parse_if() {
        let mut prog = Parser::new(&"if <(1, 5) then 8 else 9");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("if 1 < 5 then 8 else 9", format!("{}", e));
    }
}

#[cfg(test)]
mod nested_tests {

    use crate::parser::Parser;

    #[test]
    fn parse_nested_binary_expression() {
        let mut prog = Parser::new(&"+(1, -(2, 3))");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("1 + 2 - 3", format!("{}", e));
    }

    #[test]
    fn parse_nested_apply_expression() {
        let mut prog = Parser::new(&"apply(func x => -(x, 2), 5)");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("func x => x - 2 (5)", format!("{}", e));
    }

    #[test]
    fn parse_nested_if_expression() {
        let mut prog = Parser::new(&"if <(1, 5) then if <(2, 3) then 2 else 3 else 4");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!(
            "if 1 < 5 then if 2 < 3 then 2 else 3 else 4",
            format!("{}", e)
        );
    }

    #[test]
    fn parse_nested_complex_expression() {
        let mut prog = Parser::new(&"apply(func x => if <(x, 10) then -(10, x) else +(x, 10), 5)");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!(
            "func x => if x < 10 then 10 - x else x + 10 (5)",
            format!("{}", e)
        );
    }

    #[test]
    fn parse_nested_multiple_ifs() {
        let mut prog =
            Parser::new(&"if <(1, 5) then if <(2, 3) then 2 else 3 else if <(4, 6) then 6 else 4");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!(
            "if 1 < 5 then if 2 < 3 then 2 else 3 else if 4 < 6 then 6 else 4",
            format!("{}", e)
        );
    }
}

#[cfg(test)]
mod eval_tests {

    use crate::expression::{BinaryOperator, Expression, UnaryOperator};

    #[test]
    fn eval_integer() {
        let expr = Expression::Integer(42);
        let result = expr.eval();
        assert_eq!(result, Ok(Expression::Integer(42)));
    }

    #[test]
    fn eval_variable() {
        let expr = Expression::Variable("x".to_string());
        let result = expr.eval();
        assert_eq!(result, Ok(Expression::Variable("x".to_string())));
    }

    #[test]
    fn eval_boolean() {
        let expr = Expression::Boolean(true);
        let result = expr.eval();
        assert_eq!(result, Ok(Expression::Boolean(true)));
    }

    #[test]
    fn eval_not_true() {
        let expr = Expression::UnaryOp {
            op: UnaryOperator::Not,
            child: Box::new(Expression::Boolean(true)),
        };
        let result = expr.eval();
        assert_eq!(result, Ok(Expression::Boolean(false)));
    }

    #[test]
    fn eval_not_false() {
        let expr = Expression::UnaryOp {
            op: UnaryOperator::Not,
            child: Box::new(Expression::Boolean(false)),
        };
        let result = expr.eval();
        assert_eq!(result, Ok(Expression::Boolean(true)));
    }

    #[test]
    fn eval_addition() {
        let expr = Expression::BinaryOp {
            op: BinaryOperator::Add,
            lhs: Box::new(Expression::Integer(2)),
            rhs: Box::new(Expression::Integer(3)),
        };
        let result = expr.eval();
        assert_eq!(result, Ok(Expression::Integer(5)));
    }

    #[test]
    fn eval_subtraction() {
        let expr = Expression::BinaryOp {
            op: BinaryOperator::Subtract,
            lhs: Box::new(Expression::Integer(8)),
            rhs: Box::new(Expression::Integer(3)),
        };
        let result = expr.eval();
        assert_eq!(result, Ok(Expression::Integer(5)));
    }

    #[test]
    fn eval_multiplication() {
        let expr = Expression::BinaryOp {
            op: BinaryOperator::Multiply,
            lhs: Box::new(Expression::Integer(2)),
            rhs: Box::new(Expression::Integer(3)),
        };
        let result = expr.eval();
        assert_eq!(result, Ok(Expression::Integer(6)));
    }

    #[test]
    fn eval_division() {
        let expr = Expression::BinaryOp {
            op: BinaryOperator::Divide,
            lhs: Box::new(Expression::Integer(10)),
            rhs: Box::new(Expression::Integer(2)),
        };
        let result = expr.eval();
        assert_eq!(result, Ok(Expression::Integer(5)));
    }

    #[test]
    fn eval_less_than_true() {
        let expr = Expression::BinaryOp {
            op: BinaryOperator::LessThan,
            lhs: Box::new(Expression::Integer(3)),
            rhs: Box::new(Expression::Integer(5)),
        };
        let result = expr.eval();
        assert_eq!(result, Ok(Expression::Boolean(true)));
    }

    #[test]
    fn eval_less_than_false() {
        let expr = Expression::BinaryOp {
            op: BinaryOperator::LessThan,
            lhs: Box::new(Expression::Integer(8)),
            rhs: Box::new(Expression::Integer(5)),
        };
        let result = expr.eval();
        assert_eq!(result, Ok(Expression::Boolean(false)));
    }

    #[test]
    fn eval_equals_true() {
        let expr = Expression::BinaryOp {
            op: BinaryOperator::Equals,
            lhs: Box::new(Expression::Integer(4)),
            rhs: Box::new(Expression::Integer(4)),
        };
        let result = expr.eval();
        assert_eq!(result, Ok(Expression::Boolean(true)));
    }

    #[test]
    fn eval_equals_false() {
        let expr = Expression::BinaryOp {
            op: BinaryOperator::Equals,
            lhs: Box::new(Expression::Integer(2)),
            rhs: Box::new(Expression::Integer(5)),
        };
        let result = expr.eval();
        assert_eq!(result, Ok(Expression::Boolean(false)));
    }
}

#[cfg(test)]
mod nested_eval_tests {
    use crate::expression::{BinaryOperator, Expression};

    #[test]
    fn eval_nested_addition() {
        // Test: +(1, +(2, 3))
        let expression = Expression::BinaryOp {
            op: BinaryOperator::Add,
            lhs: Box::new(Expression::Integer(1)),
            rhs: Box::new(Expression::BinaryOp {
                op: BinaryOperator::Add,
                lhs: Box::new(Expression::Integer(2)),
                rhs: Box::new(Expression::Integer(3)),
            }),
        };
        let result = expression.eval();
        assert!(result.is_ok());
        assert_eq!(Expression::Integer(6), result.unwrap());
    }

    #[test]
    fn eval_nested_subtraction() {
        // Test: -(10, -(5, 3))
        let expression = Expression::BinaryOp {
            op: BinaryOperator::Subtract,
            lhs: Box::new(Expression::Integer(10)),
            rhs: Box::new(Expression::BinaryOp {
                op: BinaryOperator::Subtract,
                lhs: Box::new(Expression::Integer(5)),
                rhs: Box::new(Expression::Integer(3)),
            }),
        };
        let result = expression.eval();
        assert!(result.is_ok());
        assert_eq!(Expression::Integer(8), result.unwrap());
    }

    #[test]
    fn eval_nested_multiplication() {
        // Test: *(3, *(2, 4))
        let expression = Expression::BinaryOp {
            op: BinaryOperator::Multiply,
            lhs: Box::new(Expression::Integer(3)),
            rhs: Box::new(Expression::BinaryOp {
                op: BinaryOperator::Multiply,
                lhs: Box::new(Expression::Integer(2)),
                rhs: Box::new(Expression::Integer(4)),
            }),
        };
        let result = expression.eval();
        assert!(result.is_ok());
        assert_eq!(Expression::Integer(24), result.unwrap());
    }

    #[test]
    fn eval_nested_division() {
        // Test: /(15, /(6, 2))
        let expression = Expression::BinaryOp {
            op: BinaryOperator::Divide,
            lhs: Box::new(Expression::Integer(15)),
            rhs: Box::new(Expression::BinaryOp {
                op: BinaryOperator::Divide,
                lhs: Box::new(Expression::Integer(6)),
                rhs: Box::new(Expression::Integer(2)),
            }),
        };
        let result = expression.eval();
        assert!(result.is_ok());
        assert_eq!(Expression::Integer(5), result.unwrap());
    }

    #[test]
    fn eval_nested_and() {
        // Test: &(T, &(F, T))
        let expression = Expression::BinaryOp {
            op: BinaryOperator::And,
            lhs: Box::new(Expression::Boolean(true)),
            rhs: Box::new(Expression::BinaryOp {
                op: BinaryOperator::And,
                lhs: Box::new(Expression::Boolean(false)),
                rhs: Box::new(Expression::Boolean(true)),
            }),
        };
        let result = expression.eval();
        assert!(result.is_ok());
        assert_eq!(Expression::Boolean(false), result.unwrap());
    }

    #[test]
    fn eval_nested_or() {
        // Test: |(T, |(F, T))
        let expression = Expression::BinaryOp {
            op: BinaryOperator::Or,
            lhs: Box::new(Expression::Boolean(true)),
            rhs: Box::new(Expression::BinaryOp {
                op: BinaryOperator::Or,
                lhs: Box::new(Expression::Boolean(false)),
                rhs: Box::new(Expression::Boolean(true)),
            }),
        };
        let result = expression.eval();
        assert!(result.is_ok());
        assert_eq!(Expression::Boolean(true), result.unwrap());
    }
}

#[cfg(test)]
mod apply_tests {
    use crate::expression::Expression;
    use crate::parser::Parser;

    #[test]
    fn eval_apply_addition() {
        let mut prog = Parser::new("apply(func x => +(x, 1), 2)");
        let result = prog.parse().unwrap().eval();
        assert_eq!(result, Ok(Expression::Integer(3)));
    }

    #[test]
    fn eval_apply_subtraction() {
        let mut prog = Parser::new("apply(func x => -(x, 2), 5)");
        let result = prog.parse().unwrap().eval();
        assert_eq!(result, Ok(Expression::Integer(3)));
    }

    #[test]
    fn eval_apply_multiplication() {
        let mut prog = Parser::new("apply(func x => *(x, 3), 4)");
        let result = prog.parse().unwrap().eval();
        assert_eq!(result, Ok(Expression::Integer(12)));
    }

    #[test]
    fn eval_apply_division() {
        let mut prog = Parser::new("apply(func x => /(x, 2), 10)");
        let result = prog.parse().unwrap().eval();
        assert_eq!(result, Ok(Expression::Integer(5)));
    }

    #[test]
    fn eval_apply_equals() {
        let mut prog = Parser::new("apply(func x => =(x, 3), 3)");
        let result = prog.parse().unwrap().eval();
        assert_eq!(result, Ok(Expression::Boolean(true)));
    }

    #[test]
    fn eval_apply_less_than() {
        let mut prog = Parser::new("apply(func x => <(x, 5), 3)");
        let result = prog.parse().unwrap().eval();
        assert_eq!(result, Ok(Expression::Boolean(true)));
    }

    #[test]
    fn eval_apply_and() {
        let mut prog = Parser::new("apply(func x => &(x, T), F)");
        let result = prog.parse().unwrap().eval();
        assert_eq!(result, Ok(Expression::Boolean(false)));
    }

    #[test]
    fn eval_apply_or() {
        let mut prog = Parser::new("apply(func x => |(x, T), F)");
        let result = prog.parse().unwrap().eval();
        assert_eq!(result, Ok(Expression::Boolean(true)));
    }

    #[test]
    fn eval_apply_not() {
        let mut prog = Parser::new("apply(func x => !x, T)");
        let result = prog.parse().unwrap().eval();
        assert_eq!(result, Ok(Expression::Boolean(false)));
    }
}

#[cfg(test)]
mod if_expression_tests {
    use crate::expression::Expression;
    use crate::parser::Parser;

    #[test]
    fn eval_if_true() {
        // if T then 2 else 3
        let mut prog = Parser::new("if T then 2 else 3");
        let result = prog.parse().unwrap().eval();
        assert_eq!(result, Ok(Expression::Integer(2)));
    }

    #[test]
    fn eval_if_false() {
        // if F then 2 else 3
        let mut prog = Parser::new("if F then 2 else 3");
        let result = prog.parse().unwrap().eval();
        assert_eq!(result, Ok(Expression::Integer(3)));
    }

    #[test]
    fn eval_nested_if() {
        // if <(2, 3) then if T then 4 else 5 else 6
        let mut prog = Parser::new("if <(2, 3) then if T then 4 else 5 else 6");
        let result = prog.parse().unwrap().eval();
        assert_eq!(result, Ok(Expression::Integer(4)));
    }
}
