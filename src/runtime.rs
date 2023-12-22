/* ο（´･ω･｀o）*/

use crate::node::*;
use itertools::{self, Itertools};

pub fn execute(prg: Program) {
    let mut nodes = prg.stmts;
    let mut edges = shortest_hamiltonian_path(&nodes);
    let mut current: usize = 0;

    while nodes[current].expr != Expr::Exit {
        if execute_node(&mut nodes, current) {
            edges = shortest_hamiltonian_path(&nodes);
        }
        if let Some(next) = edges.get(edges.iter().position(|n| *n == current).unwrap() + 1) {
            current = *next;
        } else {
            break;
        }
    }
}

/// returns true if a recalculateion of the path is needed.
fn execute_node(nodes: &mut Vec<Node>, current: usize) -> bool {
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

    false
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
