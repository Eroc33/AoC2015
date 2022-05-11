use std::collections::HashMap;

use shared::combine::{parser::char::{string, letter}, Parser, Stream, attempt, choice, many1, sep_by1};


#[derive(Debug,Copy,Clone)]
pub enum BinaryOp {
    And,
    Or,
    LShift,
    RShift,
}

impl BinaryOp {
    pub fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: Stream<Token = char>,
    {
        choice((
            string("AND").map(|_| BinaryOp::And),
            string("OR").map(|_| BinaryOp::Or),
            string("LSHIFT").map(|_| BinaryOp::LShift),
            string("RSHIFT").map(|_| BinaryOp::RShift),
        ))
    }

    pub fn apply(&self, l: Option<u16>, r: Option<u16>) -> Option<u16>
    {
        let l = l?;
        let r = r?;
        Some(match self {
            BinaryOp::And => l & r,
            BinaryOp::Or => l | r,
            BinaryOp::LShift => l << r,
            BinaryOp::RShift => l >> r,
        })
    }
}

#[derive(Debug,Copy,Clone)]
pub enum UnaryOp {
    Not,
}

impl UnaryOp {
    pub fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: Stream<Token = char>,
    {
        string("NOT").map(|_| UnaryOp::Not)
    }

    pub fn apply(&self, val: Option<u16>) -> Option<u16>
    {
        let val = val?;
        Some(match self {
            UnaryOp::Not => !val
        })
    }
}

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct WireName(pub String);

impl<'a> From<&'a str> for WireName{
    fn from(name: &'a str) -> Self {
        Self(name.to_owned())
    }
}

impl WireName {
    pub fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: Stream<Token = char>,
    {
        many1(letter()).map(WireName)
    }

    pub fn evaluate(&self, ctxt: &mut EvaluationContext, instructions: &Instructions) -> Option<u16>
    {
        ctxt.evaluate_wire(instructions, self)
    }
}

#[derive(Debug,Clone)]
pub enum ValueSource {
    Constant(u16),
    Wire(WireName),
}

impl ValueSource {
    pub fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: Stream<Token = char>,
    {
        choice((
            attempt(shared::parse::u16().map(|cst| ValueSource::Constant(cst) )),
            attempt(WireName::parser().map(|name| ValueSource::Wire(name) )),
        ))
    }

    fn evaluate(&self, ctxt: &mut EvaluationContext, instructions: &Instructions) -> Option<u16>
    {
        match self {
            ValueSource::Constant(val) => Some(*val),
            ValueSource::Wire(name) => name.evaluate(ctxt, instructions)
        }
    }
}

#[derive(Debug,Clone)]
pub enum Expression {
    Binary(ValueSource, BinaryOp, ValueSource),
    Unary(UnaryOp, ValueSource),
    ValueSource(ValueSource),
}

impl Expression {
    pub fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: Stream<Token = char>,
    {
        choice((
            attempt((ValueSource::parser(), string(" "), BinaryOp::parser(), string(" "), ValueSource::parser()).map(|(l,_,op,_,r)| Expression::Binary(l, op, r)  )),
            attempt((UnaryOp::parser(), string(" "), ValueSource::parser()).map(|(op,_,vs)| Expression::Unary(op, vs)  )),
            attempt((ValueSource::parser()).map(|val| Expression::ValueSource(val)  )),
        ))
    }

    fn evaluate(&self, ctxt: &mut EvaluationContext, instructions: &Instructions) -> Option<u16>
    {
        match self{
            Expression::Binary(l, op, r) => {
                let l = l.evaluate(ctxt, instructions);
                let r = r.evaluate(ctxt, instructions);
                op.apply(l,r)
            }
            Expression::Unary(op, expr) => {
                let val = expr.evaluate(ctxt, instructions);
                op.apply(val)
            }
            Expression::ValueSource(src) => {
                src.evaluate(ctxt, instructions)
            }
        }
    }
}

#[derive(Debug,Clone)]
pub struct Instruction {
    pub expression: Expression,
    pub target: WireName,
}

impl Instruction {
    pub fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: Stream<Token = char>,
    {
        (Expression::parser(), string(" -> "), WireName::parser())
            .map(|(expression, _, target)| Self { expression, target })
    }
}

pub struct Instructions{
    instructions_by_target: HashMap<WireName,Expression>
}

impl Instructions{
    pub fn parser<Input>() -> impl Parser<Input, Output = Self>
    where
        Input: Stream<Token = char>,
    {
        sep_by1(Instruction::parser(), shared::parse::lax_newline()).map(|instructions| Self::new(instructions))
    }
    pub fn new(instructions: Vec<Instruction>) -> Self{
        let instructions_by_target: HashMap<WireName,Expression> = instructions.into_iter().map(|ins| (ins.target, ins.expression)).collect();
        Self{
            instructions_by_target,
        }
    }

    pub fn wirenames(&self) -> impl Iterator<Item=&WireName>
    {
        self.instructions_by_target.keys()
    }
}

pub struct EvaluationContext{
    values: HashMap<WireName, Option<u16>>,
}


impl EvaluationContext{
    pub fn new() -> Self{
        Self{
            values: Default::default(),
        }
    }

    fn evaluate_wire(&mut self, instructions: &Instructions, wire: &WireName) -> Option<u16>
    {
        if let Some(cached) = self.values.get(wire).copied(){
            return cached;
        }
        match instructions.instructions_by_target.get(&wire){
            Some(expression) =>{
                let value = expression.evaluate(self, instructions);
                self.values.insert(wire.clone(), value);
                value
            }
            None => None,
        }
    }
}