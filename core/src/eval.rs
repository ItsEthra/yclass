use crate::{Error, ProcessInterface, Result};
use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
#[logos(skip r"[ \t\n\f]+")]
enum Token {
    #[token("(")]
    ParenLeft,
    #[token(")")]
    ParenRight,
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

impl Token {
    fn precedence(&self) -> u32 {
        match self {
            Token::Add | Token::Sub => 2,
            Token::Div | Token::Mul => 3,
            Token::Deref => 4,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Number(usize),
    Operator(Token),
}

impl Value {
    pub fn number(text: &str) -> Result<Self> {
        usize::from_str_radix(text.strip_prefix("0x").unwrap_or(text), 16)
            .map(Self::Number)
            .map_err(|_| Error::AddrEval)
    }

    pub fn rva(text: &str, process: &dyn ProcessInterface) -> Result<Self> {
        let (name, rva) = text.split_once(":$").unwrap();

        let name = if name.is_empty() { None } else { Some(name) };
        let offset = usize::from_str_radix(rva.strip_prefix("0x").unwrap_or(rva), 16)
            .map_err(|_| Error::AddrEval)?;
        let base = process
            .module_base(name)
            .map_err(|err| Error::Custom(err))?;
        Ok(Self::Number(offset + base))
    }
}

// Evaulates the expression using shunting yard algorithm
pub fn eval(expr: &str, process: &dyn ProcessInterface) -> Result<usize> {
    let mut output: Vec<Value> = vec![];
    let mut operators: Vec<Token> = vec![];

    let mut lex = Token::lexer(expr);
    while let Some(token) = lex.next() {
        let token = token.map_err(|_| Error::AddrEval)?;
        let span = &expr[lex.span()];

        match token {
            Token::Num => output.push(Value::number(span)?),
            Token::Rva => output.push(Value::rva(span, process)?),
            Token::Add | Token::Sub | Token::Div | Token::Mul | Token::Deref => {
                while let Some(last) = operators.last() {
                    if !matches!(last, Token::ParenLeft) && last.precedence() >= token.precedence()
                    {
                        output.push(Value::Operator(operators.pop().unwrap()))
                    } else {
                        break;
                    }
                }

                operators.push(token);
            }

            Token::ParenLeft => operators.push(token),
            Token::ParenRight => {
                while let Some(last) = operators.last() {
                    if !matches!(last, Token::ParenLeft) {
                        output.push(Value::Operator(operators.pop().unwrap()));
                    } else {
                        break;
                    }
                }

                if !matches!(operators.pop().ok_or(Error::AddrEval)?, Token::ParenLeft) {
                    return Err(Error::AddrEval);
                }
            }
        }
    }

    while let Some(op) = operators.pop() {
        if matches!(op, Token::ParenLeft) {
            return Err(Error::AddrEval);
        }

        output.push(Value::Operator(op));
    }

    let mut i = 0;
    while i < output.len() {
        let value = output[i];

        if matches!(
            value,
            Value::Operator(Token::Add | Token::Sub | Token::Mul | Token::Div)
        ) {
            let Value::Number(right) = output.remove(i - 1) else { return Err(Error::AddrEval) };
            let Value::Number(left) = output.remove(i - 2) else { return Err(Error::AddrEval) };
            let new = match value {
                Value::Operator(Token::Add) => left.wrapping_add(right),
                Value::Operator(Token::Sub) => left.wrapping_sub(right),
                Value::Operator(Token::Div) => left.wrapping_div(right),
                Value::Operator(Token::Mul) => left.wrapping_mul(right),
                _ => unreachable!(),
            };
            output[i - 2] = Value::Number(new);
            i -= 2;
        } else if matches!(value, Value::Operator(Token::Deref)) {
            todo!()
        }

        i += 1;
    }

    let Some(Value::Number(result)) = output.pop() else { return Err(Error::AddrEval); };
    Ok(result)
}
