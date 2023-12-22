/* ο（´･ω･｀o）*/

use crate::node::*;
use itertools::{self, Itertools};

pub fn execute(prg: Program) {
    let mut nodes = prg.stmts;
    let mut current: usize = 0;

    while current < nodes.len() && nodes[current].expr != Expr::Exit {
        execute_node(&mut nodes, current);
        current += 1;
    }
}

fn execute_node(nodes: &mut Vec<Node>, current: usize) {
    match nodes[current].expr {
        Expr::Nop => {}
        Expr::Exit => {}
        Expr::Display(p) => match p {
            Param::Literal(n) => print!("{n}"),
            Param::NodeIndex(i) => print!("{}", nodes[i].z),
            Param::Null => {}
        },
        Expr::DisplayChar(p) => match p {
            Param::Literal(n) => print!("{}", n as u8 as char),
            Param::NodeIndex(i) => print!("{}", nodes[i].z as u8 as char),
            Param::Null => {}
        },
        n => todo!("cant execute `{n:?}` yet"),
    }
}

fn pathlength(nodes: &Vec<Node>, indexes: &Vec<usize>) -> f64 {
    let mut sum = nodes[0].distance(&nodes[indexes[0]]);
    for i in 0..indexes.len() - 1 {
        sum += nodes[indexes[i]].distance(&nodes[indexes[i + 1]]);
    }
    sum
}
pub fn shortest_hamiltonian_path(nodes: &Vec<Node>) -> Vec<usize> {
    let mut bestpath = Vec::new();
    let mut bestlength = f64::INFINITY;
    for edges in (1..nodes.len()).permutations(nodes.len() - 1) {
        let len = pathlength(nodes, &edges);
        if len < bestlength {
            bestlength = len;
            bestpath = edges;
        }
    }

    bestpath.insert(0, 0);
    bestpath
}
