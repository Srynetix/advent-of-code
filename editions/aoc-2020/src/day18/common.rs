//! Common

use std::collections::HashMap;

use aoc_sx::maplit::hashmap;
use thiserror::Error;

/// Day error.
#[derive(Debug, Error)]
pub enum DayError {
    /// Token parse error.
    #[error("Parse token error: {0}")]
    ParseTokenError(String),

    /// Expression parse error.
    #[error("Parse expression error: {0}")]
    ParseExpressionError(String),
}

/// Expression token.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ExpressionToken {
    /// Digit.
    Digit(u32),
    /// Operator sign.
    OperatorSign(OperatorSign),
    /// Parenthese.
    Parenthese(Parenthese),
    /// End.
    End,
}

/// Operator sign.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum OperatorSign {
    /// Addition.
    Addition,
    /// Multiplication.
    Multiplication,
}

/// Parenthese.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Parenthese {
    /// Open.
    Open(usize),
    /// Close.
    Close(usize),
}

/// Expression node.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExpressionNode {
    /// Number
    Number(isize),
    /// Addition
    Addition(Box<Self>, Box<Self>),
    /// Multiplication
    Multiplication(Box<Self>, Box<Self>),
}

/// Expression lexer.
pub struct ExpressionLexer;

/// Expression parser.
pub struct ExpressionParser;

/// Expression lexer context.
#[derive(Debug, Default)]
pub struct ExpressionLexerContext {
    last_parens_index: usize,
}

impl ExpressionLexerContext {
    /// Creates a new expression lexer context.
    pub const fn new() -> Self {
        Self {
            last_parens_index: 0,
        }
    }
}

impl ExpressionLexer {
    /// Parse tokens.
    ///
    /// # Arguments
    ///
    /// * `input` - Input string
    pub fn parse_tokens(input: &str) -> Vec<ExpressionToken> {
        let mut context = ExpressionLexerContext::new();
        let mut tokens = input
            .chars()
            .filter_map(|c| {
                if c == ' ' {
                    None
                } else {
                    Some(Self::parse_token(&mut context, c))
                }
            })
            .collect::<Vec<_>>();

        tokens.push(ExpressionToken::End);
        tokens
    }

    /// Parse token.
    ///
    /// # Arguments
    ///
    /// * `input` - Input char
    pub fn parse_token(context: &mut ExpressionLexerContext, input: char) -> ExpressionToken {
        match input {
            '+' => ExpressionToken::OperatorSign(OperatorSign::Addition),
            '*' => ExpressionToken::OperatorSign(OperatorSign::Multiplication),
            '(' => {
                let node = ExpressionToken::Parenthese(Parenthese::Open(context.last_parens_index));
                context.last_parens_index += 1;
                node
            }
            ')' => {
                if context.last_parens_index == 0 {
                    panic!("Unmatching closing parenthese");
                }

                context.last_parens_index -= 1;
                ExpressionToken::Parenthese(Parenthese::Close(context.last_parens_index))
            }
            other => other.to_digit(10).map(ExpressionToken::Digit).unwrap(),
        }
    }
}

impl ExpressionParser {
    /// Parse and compute expression from input.
    ///
    /// # Arguments
    ///
    /// * `input` - Input string
    /// * `token_precedences` - Precedence map
    pub fn parse_and_compute_expression(
        input: &str,
        token_precedences: &HashMap<ExpressionToken, isize>,
    ) -> isize {
        let tokens = ExpressionLexer::parse_tokens(input);
        let tree = Self::generate_tree_from_tokens(&tokens, token_precedences);

        Self::resolve_expression_tree(tree)
    }

    /// Resolve expression tree to a number.
    ///
    /// # Arguments
    ///
    /// * `tree` - Expression node
    pub fn resolve_expression_tree(tree: ExpressionNode) -> isize {
        use ExpressionNode::{Addition, Multiplication, Number};

        match tree {
            Number(n) => n,
            Addition(a, b) => Self::resolve_expression_tree(*a) + Self::resolve_expression_tree(*b),
            Multiplication(a, b) => {
                Self::resolve_expression_tree(*a) * Self::resolve_expression_tree(*b)
            }
        }
    }

    /// Generate node tree from tokens.
    ///
    /// # Arguments
    ///
    /// * `tokens` - Token stream
    /// * `token_precedences` - Precedence map
    #[allow(clippy::cast_possible_wrap)]
    pub fn generate_tree_from_tokens(
        tokens: &[ExpressionToken],
        token_precedences: &HashMap<ExpressionToken, isize>,
    ) -> ExpressionNode {
        let mut cursor = 0;
        Self::parse_expr(tokens, &mut cursor, token_precedences)
    }

    /// Default token precedences.
    pub fn default_token_precedences() -> HashMap<ExpressionToken, isize> {
        hashmap! {
            ExpressionToken::OperatorSign(OperatorSign::Addition) => 1,
            ExpressionToken::OperatorSign(OperatorSign::Multiplication) => 1,
        }
    }

    /// Token precedences with addition priority.
    pub fn addition_token_precedences() -> HashMap<ExpressionToken, isize> {
        hashmap! {
            ExpressionToken::OperatorSign(OperatorSign::Addition) => 2,
            ExpressionToken::OperatorSign(OperatorSign::Multiplication) => 1,
        }
    }

    fn parse_expr(
        tokens: &[ExpressionToken],
        cursor: &mut usize,
        token_precedences: &HashMap<ExpressionToken, isize>,
    ) -> ExpressionNode {
        let lhs = Self::parse_lhs(tokens, cursor, token_precedences);
        Self::parse_rhs(tokens, cursor, lhs, 0, token_precedences)
    }

    fn parse_lhs(
        tokens: &[ExpressionToken],
        cursor: &mut usize,
        token_precedences: &HashMap<ExpressionToken, isize>,
    ) -> ExpressionNode {
        match Self::peek_token(tokens, cursor) {
            ExpressionToken::Parenthese(Parenthese::Open(_)) => {
                Self::parse_parens_expr(tokens, cursor, token_precedences)
            }
            ExpressionToken::Digit(_) => Self::parse_digit_expr(tokens, cursor),
            other => panic!("Unsupported lhs: {:?}", other),
        }
    }

    fn peek_token<'a>(tokens: &'a [ExpressionToken], cursor: &mut usize) -> &'a ExpressionToken {
        tokens.get(*cursor).unwrap()
    }

    fn consume_token<'a>(
        tokens: &'a [ExpressionToken],
        cursor: &mut usize,
    ) -> Option<&'a ExpressionToken> {
        let token = tokens.get(*cursor);
        *cursor += 1;
        token
    }

    fn parse_rhs(
        tokens: &[ExpressionToken],
        cursor: &mut usize,
        lhs: ExpressionNode,
        precedence: isize,
        token_precedences: &HashMap<ExpressionToken, isize>,
    ) -> ExpressionNode {
        let mut curr_lhs = lhs;

        loop {
            let token = Self::peek_token(tokens, cursor);
            let token_precedence = token_precedences.get(token).copied().unwrap_or(-1);
            if token_precedence < precedence {
                return curr_lhs;
            }

            Self::consume_token(tokens, cursor);
            let mut curr_rhs = Self::parse_lhs(tokens, cursor, token_precedences);
            let next_token = Self::peek_token(tokens, cursor);
            let next_prec = token_precedences.get(next_token).copied().unwrap_or(-1);
            if token_precedence < next_prec {
                curr_rhs =
                    Self::parse_rhs(tokens, cursor, curr_rhs, precedence + 1, token_precedences);
            }

            curr_lhs = Self::parse_operation(token, curr_lhs, curr_rhs);
        }
    }

    fn parse_operation(
        token: &ExpressionToken,
        lhs: ExpressionNode,
        rhs: ExpressionNode,
    ) -> ExpressionNode {
        match token {
            ExpressionToken::OperatorSign(OperatorSign::Addition) => {
                ExpressionNode::Addition(Box::new(lhs), Box::new(rhs))
            }
            ExpressionToken::OperatorSign(OperatorSign::Multiplication) => {
                ExpressionNode::Multiplication(Box::new(lhs), Box::new(rhs))
            }
            other => panic!("Unsupported token in operation: {:?}", other),
        }
    }

    #[allow(clippy::cast_possible_wrap)]
    fn parse_digit_expr(tokens: &[ExpressionToken], cursor: &mut usize) -> ExpressionNode {
        match Self::peek_token(tokens, cursor) {
            ExpressionToken::Digit(d) => {
                Self::consume_token(tokens, cursor);
                ExpressionNode::Number(*d as isize)
            }
            other => panic!("Unsupported digit expr: {:?}", other),
        }
    }

    fn parse_parens_expr(
        tokens: &[ExpressionToken],
        cursor: &mut usize,
        token_precedences: &HashMap<ExpressionToken, isize>,
    ) -> ExpressionNode {
        Self::consume_token(tokens, cursor);
        let expr = Self::parse_expr(tokens, cursor, token_precedences);
        match Self::peek_token(tokens, cursor) {
            ExpressionToken::Parenthese(Parenthese::Close(_)) => {
                Self::consume_token(tokens, cursor);
                expr
            }
            other => panic!("Bad token instead of close parens: {:?}", other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_token() {
        let mut context = ExpressionLexerContext::new();

        assert_eq!(
            ExpressionLexer::parse_token(&mut context, '+'),
            ExpressionToken::OperatorSign(OperatorSign::Addition)
        );
        assert_eq!(
            ExpressionLexer::parse_token(&mut context, '*'),
            ExpressionToken::OperatorSign(OperatorSign::Multiplication)
        );
        assert_eq!(
            ExpressionLexer::parse_token(&mut context, '1'),
            ExpressionToken::Digit(1)
        );
        assert_eq!(
            ExpressionLexer::parse_token(&mut context, '('),
            ExpressionToken::Parenthese(Parenthese::Open(0))
        );
        assert_eq!(
            ExpressionLexer::parse_token(&mut context, '('),
            ExpressionToken::Parenthese(Parenthese::Open(1))
        );
        assert_eq!(
            ExpressionLexer::parse_token(&mut context, ')'),
            ExpressionToken::Parenthese(Parenthese::Close(1))
        );
        assert_eq!(
            ExpressionLexer::parse_token(&mut context, ')'),
            ExpressionToken::Parenthese(Parenthese::Close(0))
        );
        assert_eq!(
            ExpressionLexer::parse_token(&mut context, '('),
            ExpressionToken::Parenthese(Parenthese::Open(0))
        );
        assert_eq!(
            ExpressionLexer::parse_token(&mut context, ')'),
            ExpressionToken::Parenthese(Parenthese::Close(0))
        );
    }

    #[test]
    fn test_parse_tokens() {
        assert_eq!(
            ExpressionLexer::parse_tokens("1 + 2 * 3 + 4 * 5 + 6"),
            vec![
                ExpressionToken::Digit(1),
                ExpressionToken::OperatorSign(OperatorSign::Addition),
                ExpressionToken::Digit(2),
                ExpressionToken::OperatorSign(OperatorSign::Multiplication),
                ExpressionToken::Digit(3),
                ExpressionToken::OperatorSign(OperatorSign::Addition),
                ExpressionToken::Digit(4),
                ExpressionToken::OperatorSign(OperatorSign::Multiplication),
                ExpressionToken::Digit(5),
                ExpressionToken::OperatorSign(OperatorSign::Addition),
                ExpressionToken::Digit(6),
                ExpressionToken::End
            ]
        );

        assert_eq!(
            ExpressionLexer::parse_tokens("1 + (2 * 3) + (4 * (5 + 6))"),
            vec![
                ExpressionToken::Digit(1),
                ExpressionToken::OperatorSign(OperatorSign::Addition),
                ExpressionToken::Parenthese(Parenthese::Open(0)),
                ExpressionToken::Digit(2),
                ExpressionToken::OperatorSign(OperatorSign::Multiplication),
                ExpressionToken::Digit(3),
                ExpressionToken::Parenthese(Parenthese::Close(0)),
                ExpressionToken::OperatorSign(OperatorSign::Addition),
                ExpressionToken::Parenthese(Parenthese::Open(0)),
                ExpressionToken::Digit(4),
                ExpressionToken::OperatorSign(OperatorSign::Multiplication),
                ExpressionToken::Parenthese(Parenthese::Open(1)),
                ExpressionToken::Digit(5),
                ExpressionToken::OperatorSign(OperatorSign::Addition),
                ExpressionToken::Digit(6),
                ExpressionToken::Parenthese(Parenthese::Close(1)),
                ExpressionToken::Parenthese(Parenthese::Close(0)),
                ExpressionToken::End
            ]
        )
    }

    #[test]
    fn test_generate_tree_from_tokens_no_parens() {
        use ExpressionNode::{Addition, Multiplication, Number};

        let first_sample = ExpressionLexer::parse_tokens("1 + 2 * 3 + 4 * 5 + 6");
        let token_precedences = ExpressionParser::default_token_precedences();

        assert_eq!(
            ExpressionParser::generate_tree_from_tokens(&first_sample, &token_precedences),
            Addition(
                Box::new(Multiplication(
                    Box::new(Addition(
                        Box::new(Multiplication(
                            Box::new(Addition(Box::new(Number(1)), Box::new(Number(2)))),
                            Box::new(Number(3))
                        )),
                        Box::new(Number(4))
                    )),
                    Box::new(Number(5))
                )),
                Box::new(Number(6))
            )
        );
    }

    #[test]
    fn test_generate_tree_from_tokens_with_parens() {
        use ExpressionNode::{Addition, Multiplication, Number};

        let second_sample = ExpressionLexer::parse_tokens("1 + (2 * 3) + (4 * (5 + 6))");
        let token_precedences = ExpressionParser::default_token_precedences();

        assert_eq!(
            ExpressionParser::generate_tree_from_tokens(&second_sample, &token_precedences),
            Addition(
                Box::new(Addition(
                    Box::new(Number(1)),
                    Box::new(Multiplication(Box::new(Number(2)), Box::new(Number(3)),))
                )),
                Box::new(Multiplication(
                    Box::new(Number(4)),
                    Box::new(Addition(Box::new(Number(5)), Box::new(Number(6))))
                ))
            )
        );
    }

    #[test]
    fn test_parse_and_compute_expression() {
        let precedences = ExpressionParser::default_token_precedences();

        assert_eq!(
            ExpressionParser::parse_and_compute_expression("1 + 2 * 3 + 4 * 5 + 6", &precedences),
            71
        );
        assert_eq!(
            ExpressionParser::parse_and_compute_expression(
                "1 + (2 * 3) + (4 * (5 + 6))",
                &precedences
            ),
            51
        );
        assert_eq!(
            ExpressionParser::parse_and_compute_expression("2 * 3 + (4 * 5)", &precedences),
            26
        );
        assert_eq!(
            ExpressionParser::parse_and_compute_expression(
                "5 + (8 * 3 + 9 + 3 * 4 * 3)",
                &precedences
            ),
            437
        );
        assert_eq!(
            ExpressionParser::parse_and_compute_expression(
                "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
                &precedences
            ),
            12240
        );
        assert_eq!(
            ExpressionParser::parse_and_compute_expression(
                "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
                &precedences
            ),
            13632
        );
    }

    #[test]
    fn test_generate_tree_from_tokens_with_another_precedences() {
        use ExpressionNode::{Addition, Multiplication, Number};

        let first_sample = ExpressionLexer::parse_tokens("1 + 2 * 3 + 4 * 5 + 6");
        let token_precedences = ExpressionParser::addition_token_precedences();

        assert_eq!(
            ExpressionParser::generate_tree_from_tokens(&first_sample, &token_precedences),
            Multiplication(
                Box::new(Addition(Box::new(Number(1)), Box::new(Number(2)),)),
                Box::new(Multiplication(
                    Box::new(Addition(Box::new(Number(3)), Box::new(Number(4)),)),
                    Box::new(Addition(Box::new(Number(5)), Box::new(Number(6)),))
                ))
            )
        );
    }
}
