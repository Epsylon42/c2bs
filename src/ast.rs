#[derive(Debug, PartialEq)]
pub enum Node {
    Block(Block),
    If(If),
    While(While),
    Nodes(Vec<Node>),
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub text: String
}

#[derive(Debug, PartialEq)]
pub struct If {
    pub cond: String,
    pub t: Box<Node>,
    pub f: Option<Box<Node>>
}

#[derive(Debug, PartialEq)]
pub struct While {
    pub cond: String,
    pub body: Box<Node>,
}

#[derive(Debug, PartialEq)]
pub struct Begin;

#[derive(Debug, PartialEq)]
pub struct End;

#[derive(Debug, PartialEq)]
pub struct Flowchart {
    pub name: String,
    pub input: Option<String>,
    pub output: Option<String>,
    pub body: Node
}
