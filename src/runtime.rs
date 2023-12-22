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
        // input and move not yet implemented
        Expr::Add(p1, p2, p3) => {
            if let Param::NodeIndex(i) = p1 {
                nodes[i].z = p2.getval(&nodes) + p3.getval(&nodes)
            } else {
                panic!("can't write to non-node");
            }
        }
        Expr::Sub(p1, p2, p3) => {
            if let Param::NodeIndex(i) = p1 {
                nodes[i].z = p2.getval(&nodes) - p3.getval(&nodes)
            } else {
                panic!("can't write to non-node");
            }
        }
        Expr::Mul(p1, p2, p3) => {
            if let Param::NodeIndex(i) = p1 {
                nodes[i].z = p2.getval(&nodes) * p3.getval(&nodes)
            } else {
                panic!("can't write to non-node");
            }
        }
        Expr::Div(p1, p2, p3) => {
            if let Param::NodeIndex(i) = p1 {
                nodes[i].z = p2.getval(&nodes) / p3.getval(&nodes)
            } else {
                panic!("can't write to non-node");
            }
        }
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
