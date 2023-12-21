pub struct Node {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub expr: Expr,
}

pub enum Expr {
    Nop,
}

pub enum Param {
    NodeIndex(usize),
    Literal(f64),
    Null,
}

impl Node {
    pub fn distance(&self, other: &Node) -> f64 {
        (
            (other.x - self.x).powi(2)
            + (other.y - self.y).powi(2)
            + (other.z - self.z).powi(2)
        ).sqrt()
    }
}