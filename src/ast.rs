use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    PInc,
    PDec,
    VInc,
    VDec,
    Output,
    Input,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Operation::PInc => write!(f, ">"),
            Operation::PDec => write!(f, "<"),
            Operation::VInc => write!(f, "+"),
            Operation::VDec => write!(f, "-"),
            Operation::Output => write!(f, "."),
            Operation::Input => write!(f, ","),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Operation {
        op: Operation,
    },
    Repetition {
        child: Box<Vec<Node>>,
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Node::Operation {op} => write!(f, "{}", op),
            Node::Repetition {child} => write!(f, "[{:?}]", child),
        }
    }
}
