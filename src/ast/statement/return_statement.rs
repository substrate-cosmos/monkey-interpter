use crate::ast::expression::integer_literal::IntegerLiteral;
use crate::ast::expression::Expression;
use crate::ast::statement::Statement;
use crate::ast::NodeInterface;
use crate::error::Error;
use crate::token::Token;
use std::any::Any;
use std::fmt::{Display, Formatter};

/// return statement
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ReturnStatement {
    pub token: Token, //  return 词法单元
    pub return_value: Box<Expression>,
}

impl ReturnStatement {
    pub fn new(token: Token) -> Self {
        Self {
            token,
            ..Default::default()
        }
    }
}

impl Default for ReturnStatement {
    fn default() -> Self {
        Self {
            token: Token::default(),
            return_value: Box::new(Expression::IntegerLiteralExpression(
                IntegerLiteral::default(),
            )),
        }
    }
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {};", self.token_literal(), self.return_value)
    }
}

impl NodeInterface for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl TryFrom<Statement> for ReturnStatement {
    type Error = anyhow::Error;

    fn try_from(value: Statement) -> Result<Self, Self::Error> {
        match value {
            Statement::Return(return_value) => Ok(return_value),
            unknow => Err(Error::UnknowStatement(unknow.to_string()).into()),
        }
    }
}
