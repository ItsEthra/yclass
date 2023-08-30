use crate::{Error, Result};
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token("(")]
    ParenOpen,
    #[token(")")]
    ParenClose,
    #[token("+")]
    Add,
    #[token("-")]
    Sub,
    #[token("/")]
    Div,
    #[token("*")]
    Mul,
    #[regex(r#"@(:(?:1|2|4|8))?"#)]
    Deref,
    #[regex(r#"(?:0x)?[\daAbBcCdDeEfF_]+"#)]
    Num,
    // TODO: Name regex is not exhaustive, fix it.
    #[regex(r#"(?:[a-zA-Z_\.]+)?:\$(?:(?:0x)?[\daAbBcCdDeEfF_]+)?"#)]
    Rva,
}

// Evaulates expression
#[allow(unused, dead_code)]
pub fn eval(expr: &str) -> Result<usize> {
    let mut lex = Token::lexer(expr);
    for token in lex {
        let token = token.map_err(|_| Error::Eval)?;
        dbg!(token);
    }

    todo!()
}

#[cfg(test)]
mod tests {
    use crate::eval;

    #[test]
    fn test_eval() {
        eval("@:8(15 + 50) + Abc:$ - 5").unwrap();
    }
}
