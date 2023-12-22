#[derive(Debug)]
pub struct Program {
    pub stmts: Vec<Node>,
}

#[derive(Debug, Clone, Copy)]
pub struct Node {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub expr: Expr,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Expr {
    Nop,
    Exit,
    Display(Param),
    DisplayChar(Param),
    Input(Param),
    InputChar(Param),
    Move(Param, Param, Param, Param),
    Add(Param, Param, Param),
    Sub(Param, Param, Param),
    Mul(Param, Param, Param),
    Div(Param, Param, Param),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Param {
    NodeIndex(usize),
    Literal(f64),
    Null,
}

impl Node {
    pub fn distance(&self, other: &Node) -> f64 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2) + (other.z - self.z).powi(2))
            .sqrt()
    }
}
