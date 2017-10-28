use std::iter::FromIterator;

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

impl FromIterator<char> for Block {
    fn from_iter<I: IntoIterator<Item=char>>(iter: I) -> Self {
        let iter = iter.into_iter();

        let mut text = match iter.size_hint() {
            (_, Some(n)) => String::with_capacity(n+10),
            (n, None) => String::with_capacity(n+10)
        };

        for c in iter {
            if c == '%' {
                text.push('\\');
            }

            text.push(c);
        }

        Block { text }
    }
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
