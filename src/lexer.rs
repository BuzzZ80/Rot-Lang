use plex::lexer;

pub struct Lexer<'a> {
    original: &'a str,
    remaining: &'a str,
}

#[derive(Debug, Clone)]
pub enum Token {
    LParen,
    RParen,
    Comma,
    IndexIndicator,
    Null,
    Number(f64),
    Ident(String),

    Whitespace,
    Comment,
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

lexer! {
    fn next_token(text: 'a) -> Token;

    r#"[ \t\n\r]"# => Token::Whitespace,
    r#"//[^\n]*"# => Token::Comment,
    r#"\("# => Token::LParen,
    r#"\)"# => Token::RParen,
    r#","# => Token::Comma,
    r#"#"# => Token::IndexIndicator,
    r#"x"# => Token::Null,
    r#"([0-9]*[.])?[0-9]+"# => {
        if let Ok(i) = text.parse() {
            Token::Number(i)
        } else {
            panic!("couldn't lex {}", text)
        }
    }
    r#"[a-zA-Z_]+"# => Token::Ident(text.to_owned()),
    r#"."# => panic!("Unexpected character `{text}`"),
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Lexer<'a> {
        Lexer {
            original: s,
            remaining: s,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token, Span);
    fn next(&mut self) -> Option<(Token, Span)> {
        loop {
            let (tok, span) = if let Some((tok, new_remaining)) = next_token(self.remaining) {
                let lo = self.original.len() - self.remaining.len();
                let hi = self.original.len() - new_remaining.len();
                self.remaining = new_remaining;
                (tok, Span { lo, hi })
            } else {
                return None;
            };
            match tok {
                Token::Whitespace | Token::Comment => {
                    continue;
                }
                tok => {
                    return Some((tok, span));
                }
            }
        }
    }
}
