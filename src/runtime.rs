/* ο（´･ω･｀o）*/

use crate::node::*;

pub fn execute(prg: Program) {
    let mut nodes = prg.stmts;
    let mut current: usize = 0;

    while current < nodes.len() && nodes[current].expr != Expr::Exit {
        execute_node(&mut nodes, current);
        current += 1;
    }
}

pub fn execute_node(nodes: &mut Vec<Node>, current: usize) {
    match nodes[current].expr {
        Expr::Nop => {}
        Expr::Exit => {}
        Expr::Display(p) => match p {
            Param::Literal(n) => print!("{n}"),
            Param::NodeIndex(i) => print!("{}", nodes[i].z),
            Param::Null => {}
        }
        Expr::DisplayChar(p) => match p {
            Param::Literal(n) => print!("{}", n as u8 as char),
            Param::NodeIndex(i) => print!("{}", nodes[i].z as u8 as char),
            Param::Null => {}
        }
        n => todo!("cant execute `{n:?}` yet"),
    }
}
