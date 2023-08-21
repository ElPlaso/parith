use crate::expression::{BinaryOperator, Expression, UnaryOperator};

#[derive(Debug, PartialEq, Clone)]
pub enum LexItem {
    OpenParen,                // "("
    CloseParen,               // ")"
    Comma,                    // ","
    Integer(i64),             // "0", "1", "2", ...
    Variable(String),         // "a", "b", "c", ...
    Boolean(bool),            // "T" or "F"
    If,                       // "if"
    Then,                     // "then"
    Else,                     // "else"
    Func,                     // "func"
    Apply,                    // "apply"
    BinaryOp(BinaryOperator), // "+", "-", "*", "/", "<", "=", "&", "|"
    UnaryOp(UnaryOperator),   // "!"
    Arrow,                    // "=>"
}

pub fn lex(input: &str) -> Result<Vec<LexItem>, String> {
    let mut result = Vec::new();

    let mut iterable = input.chars().peekable();
    while let Some(&c) = iterable.peek() {
        match c {
            '0'..='9' => {
                let mut value = String::new();
                while let Some(&c) = iterable.peek() {
                    match c {
                        '0'..='9' => {
                            value.push(c);
                            iterable.next();
                        }
                        _ => break,
                    }
                }
                result.push(LexItem::Integer(value.parse().unwrap()));
            }
            'a'..='z' => {
                let mut value = String::new();
                while let Some(&c) = iterable.peek() {
                    match c {
                        'a'..='z' => {
                            value.push(c);
                            iterable.next();
                        }
                        _ => break,
                    }
                }
                match value.as_str() {
                    "if" => result.push(LexItem::If),
                    "then" => result.push(LexItem::Then),
                    "else" => result.push(LexItem::Else),
                    "func" => result.push(LexItem::Func),
                    "apply" => result.push(LexItem::Apply),
                    _ => result.push(LexItem::Variable(value)),
                }
            }
            'T' => {
                result.push(LexItem::Boolean(true));
                iterable.next();
            }
            'F' => {
                result.push(LexItem::Boolean(false));
                iterable.next();
            }
            '+' => {
                result.push(LexItem::BinaryOp(BinaryOperator::Add));
                iterable.next();
            }
            '-' => {
                result.push(LexItem::BinaryOp(BinaryOperator::Subtract));
                iterable.next();
            }
            '*' => {
                result.push(LexItem::BinaryOp(BinaryOperator::Multiply));
                iterable.next();
            }
            '/' => {
                result.push(LexItem::BinaryOp(BinaryOperator::Divide));
                iterable.next();
            }
            '<' => {
                result.push(LexItem::BinaryOp(BinaryOperator::LessThan));
                iterable.next();
            }
            '!' => {
                result.push(LexItem::UnaryOp(UnaryOperator::Not));
                iterable.next();
            }
            '=' => {
                // Check for "=>" and "="
                iterable.next();
                if let Some(&c) = iterable.peek() {
                    match c {
                        '>' => {
                            result.push(LexItem::Arrow);
                            iterable.next();
                        }
                        _ => {
                            result.push(LexItem::BinaryOp(BinaryOperator::Equals));
                        }
                    }
                }
            }
            '&' => {
                result.push(LexItem::BinaryOp(BinaryOperator::And));
                iterable.next();
            }
            '|' => {
                result.push(LexItem::BinaryOp(BinaryOperator::Or));
                iterable.next();
            }
            '(' => {
                result.push(LexItem::OpenParen);
                iterable.next();
            }
            ',' => {
                result.push(LexItem::Comma);
                iterable.next();
            }
            ')' => {
                result.push(LexItem::CloseParen);
                iterable.next();
            }
            ' ' | '\t' => {
                // Skip whitespace
                iterable.next();
            }
            _ => {
                return Err(format!("unexpected character {}", c));
            }
        }
    }
    Ok(result)
}

pub struct Parser {
    tokens: Vec<LexItem>,
    current: usize,
}

impl Parser {
    pub fn new(program: &str) -> Self {
        let tokens = lex(program).unwrap_or_else(|err| {
            eprintln!("Error during lexing: {}", err);
            Vec::new()
        });

        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expression, String> {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        if let Some(token) = self.tokens.get(self.current) {
            match token {
                LexItem::Integer(value) => {
                    self.current += 1;
                    Ok(Expression::Integer(*value))
                }
                LexItem::Variable(name) => {
                    self.current += 1;
                    Ok(Expression::Variable(name.clone()))
                }
                LexItem::Boolean(value) => {
                    self.current += 1;
                    Ok(Expression::Boolean(*value))
                }
                LexItem::UnaryOp(op) => self.parse_unary_expression(op.clone()),
                LexItem::BinaryOp(op) => self.parse_binary_expression(op.clone()),
                LexItem::Func => self.parse_func_expression(),
                LexItem::Apply => self.parse_apply_expression(),
                LexItem::If => self.parse_if_expression(),

                _ => Err("Expected expression".to_string()),
            }
        } else {
            Err("Unexpected end of input".to_string())
        }
    }

    fn parse_unary_expression(&mut self, op: UnaryOperator) -> Result<Expression, String> {
        self.current += 1;
        let child = self.parse_expression()?;
        Ok(Expression::UnaryOp {
            op,
            child: Box::new(child),
        })
    }

    fn parse_binary_expression(&mut self, op: BinaryOperator) -> Result<Expression, String> {
        // Expect a binary operator
        if let Some(LexItem::BinaryOp(_)) = self.tokens.get(self.current) {
            self.current += 1;
        } else {
            return Err("Expected a binary operator".to_string());
        }

        // Expect an opening parenthesis '('
        if let Some(LexItem::OpenParen) = self.tokens.get(self.current) {
            self.current += 1;

            // Parse the left-hand side (lhs) expression
            let lhs = self.parse_expression()?;

            // Expect a comma ',' after the lhs
            if let Some(LexItem::Comma) = self.tokens.get(self.current) {
                self.current += 1;
            } else {
                return Err("Expected ',' after left operand of binary expression".to_string());
            }

            // Parse the right-hand side (rhs) expression
            let rhs = self.parse_expression()?;

            // Expect a closing parenthesis ')' after the rhs
            if let Some(LexItem::CloseParen) = self.tokens.get(self.current) {
                self.current += 1;
            } else {
                return Err("Expected closing parenthesis ')'".to_string());
            }

            // Construct the BinaryOp expression
            let binary_expr = Expression::BinaryOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };

            Ok(binary_expr)
        } else {
            Err(
                "Expected opening parenthesis '('. Parentheses are required for binary operations."
                    .to_string(),
            )
        }
    }

    fn parse_func_expression(&mut self) -> Result<Expression, String> {
        // Expect the "func" keyword
        if let Some(LexItem::Func) = self.tokens.get(self.current) {
            self.current += 1;
        } else {
            return Err("Expected 'func' keyword".to_string());
        }

        // Expect a variable name
        let param_name = match self.tokens.get(self.current) {
            Some(LexItem::Variable(name)) => {
                self.current += 1;
                name.clone()
            }
            _ => return Err("Expected variable name as function parameter".to_string()),
        };

        // Expect the "=>" arrow
        if let Some(LexItem::Arrow) = self.tokens.get(self.current) {
            self.current += 1;
        } else {
            return Err("Expected '=>' arrow after function parameter".to_string());
        }

        // Parse the body expression
        let body_expr = self.parse_expression()?;

        // Construct the Func expression
        let func_expr = Expression::Func {
            param: param_name,
            body: Box::new(body_expr),
        };

        Ok(func_expr)
    }

    fn parse_apply_expression(&mut self) -> Result<Expression, String> {
        // Expect the "apply" keyword
        if let Some(LexItem::Apply) = self.tokens.get(self.current) {
            self.current += 1;
        } else {
            return Err("Expected 'apply' keyword".to_string());
        }

        // Expect an opening parenthesis '('
        if let Some(LexItem::OpenParen) = self.tokens.get(self.current) {
            self.current += 1;
        } else {
            return Err(
                "Expected opening parenthesis '('. Parentheses are required for apply expression"
                    .to_string(),
            );
        }

        // Parse the function expression
        let func_expr = self.parse_expression()?;

        // Expect a comma ','
        if let Some(LexItem::Comma) = self.tokens.get(self.current) {
            self.current += 1;
        } else {
            return Err("Expected comma ',' after function expression".to_string());
        }

        // Parse the argument expression
        let arg_expr = self.parse_expression()?;

        // Expect a closing parenthesis ')'
        if let Some(LexItem::CloseParen) = self.tokens.get(self.current) {
            self.current += 1;
        } else {
            return Err(
                "Expected closing parenthesis ')'. Parentheses are required for apply expression"
                    .to_string(),
            );
        }

        // Construct the Apply expression
        let apply_expr = Expression::Apply {
            func_expr: Box::new(func_expr),
            arg_expr: Box::new(arg_expr),
        };

        Ok(apply_expr)
    }

    fn parse_if_expression(&mut self) -> Result<Expression, String> {
        // Expect the "if" keyword
        if let Some(LexItem::If) = self.tokens.get(self.current) {
            self.current += 1;
        } else {
            return Err("Expected 'if' keyword".to_string());
        }

        // Parse the condition expression
        let condition_expr = self.parse_expression()?;

        // Expect the "then" keyword
        if let Some(LexItem::Then) = self.tokens.get(self.current) {
            self.current += 1;
        } else {
            return Err("Expected 'then' keyword".to_string());
        }

        // Parse the true branch expression
        let true_expr = self.parse_expression()?;

        // Expect the "else" keyword
        if let Some(LexItem::Else) = self.tokens.get(self.current) {
            self.current += 1;
        } else {
            return Err("Expected 'else' keyword".to_string());
        }

        // Parse the false branch expression
        let false_expr = self.parse_expression()?;

        // Construct the If expression
        let if_expr = Expression::If {
            condition: Box::new(condition_expr),
            then_expr: Box::new(true_expr),
            else_expr: Box::new(false_expr),
        };

        Ok(if_expr)
    }
}
