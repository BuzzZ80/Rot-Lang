use crate::lexer::Token::*;
use crate::lexer::*;
use crate::node::*;
use plex::parser;

parser! {
    fn parse_(Token, Span);

    (a, b) {
        Span {
            lo: a.lo,
            hi: b.hi,
        }
    }

    program: Program {
        statements[s] => Program { stmts: s }
    }

    statements: Vec<Node> {
        => vec![],
        statements[mut st] LParen Number(x) Comma Number(y) Comma Number(z) Comma expression[expr] RParen => {
            st.push(Node { x, y, z, expr });
            st
        }
    }

    expression: Expr {
        Ident(op) => match op.as_str() {
            "nop" => Expr::Nop,
            "exit" => Expr::Exit,
            s => panic!("{s} does not exist, or does not take `0` parameters"),
        },
        Ident(op) param[p1] => match op.as_str() {
            "display" => Expr::Display(p1),
            "displaychar" => Expr::DisplayChar(p1),
            "input" => Expr::Input(p1),
            "inputchar" => Expr::InputChar(p1),
            s => panic!("{s} does not exist, or does not take `1` parameter"),
        },
        Ident(op) param[p1] param[p2] param[p3] => match op.as_str() {
            "add" => Expr::Add(p1, p2, p3),
            "subtract" => Expr::Sub(p1, p2, p3),
            "multiply" => Expr::Mul(p1, p2, p3),
            "divide" => Expr::Div(p1, p2, p3),
            s => panic!("{s} does not exist, or does not take `3` parameters"),
        },
        Ident(op) param[p1] param[p2] param[p3] param[p4] => match op.as_str() {
            "move" => Expr::Move(p1, p2, p3, p4),
            s => panic!("{s} does not exist, or does not take `4` parameters"),
        }
    }
    param: Param {
        Null => Param::Null,
        IndexIndicator Number(n) => {
            if n != n.floor() || n.is_sign_negative() { panic!("Expected positive integer, but was given a negative or float"); }
            Param::NodeIndex(n as usize)
        }
        Number(n) => Param::Literal(n)
    }
}

pub fn parse<I: Iterator<Item = (Token, Span)>>(
    i: I,
) -> Result<Program, (Option<(Token, Span)>, &'static str)> {
    parse_(i)
}
