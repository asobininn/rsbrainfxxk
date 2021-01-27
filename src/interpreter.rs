use crate::{Node, Operation, Result, parser};
use std::{u8, char, io::{self, Read}};

const AREA_SIZE: usize = 256;
type ProgramArea = [u8; AREA_SIZE];
type Output = Result<()>;

pub struct Interpreter {
    area: ProgramArea,
    now: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            area: [0; AREA_SIZE],
            now: 0
        }
    }

    pub fn from_ast(&mut self, ast: Vec<Node>) -> Output {
        let eval = Eval::new();
        for node in ast {
            eval.eval(&node, &mut self.area, &mut self.now);
        }
        Ok(())
    }

    pub fn from_source(&mut self, source: &str) -> Output {
        let ast: Vec<Node> = parser::parse(source).unwrap();
        self.from_ast(ast)?;
        Ok(())
    }

}

struct Eval;

impl Eval {
    pub fn new() -> Self { Self }

    pub fn eval(&self, node: &Node, area: &mut [u8], now: &mut usize) {
        match node {
            Node::Operation { op } => {
                match op {
                    Operation::PInc => {
                        *now += 1;
                        if *now == AREA_SIZE { *now = 0 }
                    },
                    Operation::PDec => {
                        if *now == 0 { *now = AREA_SIZE }
                        *now -= 1;
                    },
                    Operation::VInc => {
                        if area[*now] == u8::MAX - 1 { area[*now] = 0 }
                        else { area[*now] += 1 }
                    },
                    Operation::VDec => {
                        if area[*now] == 0 { area[*now] = u8::MAX - 1 }
                        else { area[*now] -= 1 }
                    },
                    Operation::Output => print!("{}", area[*now] as char),
                    Operation::Input => {
                        let input = io::stdin()
                            .bytes()
                            .next()
                            .and_then(|r| r.ok())
                            .map(|b| b as u8)
                            .unwrap();
                        area[*now] = input;
                    },
                }
            },
            Node::Repetition { child } => {
                while area[*now] != 0 {
                    for node in child.iter() {
                        self.eval(node, area, now);
                    }
                }
            },
        }
    }
}
