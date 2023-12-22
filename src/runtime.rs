/* ο（´･ω･｀o）*/

use std::io::Write;

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
        Expr::Nop => return false,
        Expr::Exit => return false,
        Expr::Display(p) => {
            match p {
                Param::Literal(n) => print!("{n}"),
                Param::NodeIndex(i) => print!("{}", nodes[i].z),
                Param::Null => {}
            }
            let _ = std::io::stdout().flush();
            return false;
        }
        Expr::DisplayChar(p) => {
            match p {
                Param::Literal(n) => print!("{}", n as u8 as char),
                Param::NodeIndex(i) => print!("{}", nodes[i].z as u8 as char),
                Param::Null => {}
            }
            let _ = std::io::stdout().flush();
            return false;
        }
        Expr::Input(p) => match p {
            Param::NodeIndex(i) => {
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("failed to read line from user");
                nodes[i].z = input
                    .chars()
                    .take_while(|c| c.is_numeric() || *c == '.' || *c == '-')
                    .collect::<String>()
                    .parse()
                    .expect("Invalid number input");
            }
            _ => panic!("Expected node index, found {:?}", p),
        },
        Expr::InputChar(p) => match p {
            Param::NodeIndex(i) => {
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("failed to read line from user");
                nodes[i].z = input.chars().next().expect("input was empty") as u8 as f64;
            }
            _ => panic!("Expected node index, found {:?}", p),
        },
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
        Expr::Move(p1, p2, p3, p4) => {
            if let Param::NodeIndex(i) = p1 {
                if p2 != Param::Null {
                    nodes[i].x = p2.getval(nodes);
                }
                if p3 != Param::Null {
                    nodes[i].y = p3.getval(nodes);
                }
                if p4 != Param::Null {
                    nodes[i].z = p4.getval(nodes);
                }
                return true;
            } else {
                panic!("can't write to non-node");
            }
        }
        n => todo!("cant execute `{n:?}` yet"),
    }
    true
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
