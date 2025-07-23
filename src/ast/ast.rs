#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Number(f64),
    String(String),
    Boolean(bool),

    Identifier(String),
    List(Vec<Expression>),
}
