// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::lexer::preprocessor::context::PreprocContext;
use crate::lexer::{Lexer, LocToken, Token};
use crate::parser::attributes::Attributes;
use crate::parser::literals::StringLiteralParser;

#[derive(Clone, Debug, PartialEq)]
pub struct Asm {
    pub attributes: Option<Attributes>,
    pub code: String,
}

struct AsmParser<'a, 'b, PC: PreprocContext> {
    lexer: &'b mut Lexer<'a, PC>,
}

impl<'a, 'b, PC: PreprocContext> AsmParser<'a, 'b, PC> {
    fn new(lexer: &'b mut Lexer<'a, PC>) -> Self {
        Self { lexer }
    }

    fn parse(self, attributes: Option<Attributes>) -> (Option<LocToken>, Option<Asm>) {
        let tok = self.lexer.next_useful();
        if tok.tok != Token::LeftParen {
            unreachable!("Invalid token in asm declaration: {:?}", tok);
        }

        let tok = self.lexer.next_useful();

        if let Some(code) = tok.tok.get_string() {
            let slp = StringLiteralParser::new(self.lexer);
            let (tok, code) = slp.parse(&code);

            let tok = tok.unwrap_or_else(|| self.lexer.next_useful());
            if tok.tok != Token::RightParen {
                unreachable!("Invalid token in asm declaration: {:?}", tok);
            }

            (None, Some(Asm { attributes, code }))
        } else {
            unreachable!("Invalid token in asm declaration");
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::lexer::preprocessor::context::DefaultContext;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_asm() {
        let mut l = Lexer::<DefaultContext>::new(
            r#"
(R"(
.globl func
    .type func, @function
    func:
    .cfi_startproc
    movl $7, %eax
    ret
    .cfi_endproc
)")
"#
            .as_bytes(),
        );
        let p = AsmParser::new(&mut l);
        let (_, u) = p.parse(None);

        let code = r#"
.globl func
    .type func, @function
    func:
    .cfi_startproc
    movl $7, %eax
    ret
    .cfi_endproc
"#;

        assert_eq!(
            u.unwrap(),
            Asm {
                attributes: None,
                code: code.to_string(),
            }
        );
    }
}