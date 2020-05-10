// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::lexer::lexer::{Lexer, Token};
use crate::lexer::preprocessor::context::PreprocContext;
use crate::parser::attributes::Attributes;
use crate::parser::expressions::{ExprNode, ExpressionParser};

use crate::dump_obj;
use crate::parser::dump::Dump;
use termcolor::StandardStreamLock;

#[derive(Clone, Debug, PartialEq)]
pub struct Return {
    pub(crate) attributes: Option<Attributes>,
    pub(crate) val: Option<ExprNode>,
}

impl Dump for Return {
    fn dump(&self, name: &str, prefix: &str, last: bool, stdout: &mut StandardStreamLock) {
        dump_obj!(self, name, "return", prefix, last, stdout, attributes, val);
    }
}

pub struct ReturnStmtParser<'a, 'b, PC: PreprocContext> {
    lexer: &'b mut Lexer<'a, PC>,
}

impl<'a, 'b, PC: PreprocContext> ReturnStmtParser<'a, 'b, PC> {
    pub(super) fn new(lexer: &'b mut Lexer<'a, PC>) -> Self {
        Self { lexer }
    }

    pub(super) fn parse(self, attributes: Option<Attributes>) -> (Option<Token>, Option<Return>) {
        let mut ep = ExpressionParser::new(self.lexer, Token::Eof);
        let (tok, expr) = ep.parse(None);

        (
            tok,
            Some(Return {
                attributes,
                val: expr,
            }),
        )
    }
}
