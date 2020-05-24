// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::rc::Rc;
use termcolor::StandardStreamLock;

use super::{Statement, StatementParser};
use crate::lexer::lexer::{TLexer, Token};
use crate::parser::attributes::Attributes;
use crate::parser::declarations::{DeclOrExpr, DeclOrExprParser};
use crate::parser::dump::Dump;
use crate::parser::errors::ParserError;
use crate::parser::{Context, ScopeKind};

#[derive(Clone, Debug, PartialEq)]
pub struct While {
    pub attributes: Option<Attributes>,
    pub condition: DeclOrExpr,
    pub body: Statement,
}

impl Dump for While {
    fn dump(&self, name: &str, prefix: &str, last: bool, stdout: &mut StandardStreamLock) {
        dump_obj!(self, name, "while", prefix, last, stdout, attributes, condition, body);
    }
}

pub struct WhileStmtParser<'a, L: TLexer> {
    lexer: &'a mut L,
}

impl<'a, L: TLexer> WhileStmtParser<'a, L> {
    pub(super) fn new(lexer: &'a mut L) -> Self {
        Self { lexer }
    }

    pub(super) fn parse(
        self,
        attributes: Option<Attributes>,
        context: &mut Context,
    ) -> Result<(Option<Token>, Option<While>), ParserError> {
        let tok = self.lexer.next_useful();

        if tok != Token::LeftParen {
            return Err(ParserError::InvalidTokenInWhile {
                sp: self.lexer.span(),
                tok,
            });
        }

        context.set_current(None, ScopeKind::WhileBlock);

        let dep = DeclOrExprParser::new(self.lexer);
        let (tok, condition) = dep.parse(None, context)?;

        if let Some(DeclOrExpr::Decl(typ)) = condition.as_ref() {
            context.add_type_decl(Rc::clone(typ));
        }

        let tok = tok.unwrap_or_else(|| self.lexer.next_useful());
        if tok != Token::RightParen {
            context.pop();
            return Err(ParserError::InvalidTokenInWhile {
                sp: self.lexer.span(),
                tok,
            });
        }

        let sp = StatementParser::new(self.lexer);
        let (tok, body) = sp.parse(None, context)?;
        context.pop();

        Ok((
            tok,
            Some(While {
                attributes,
                condition: condition.unwrap(),
                body: body.unwrap(),
            }),
        ))
    }
}
