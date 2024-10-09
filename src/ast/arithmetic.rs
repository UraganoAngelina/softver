use crate::ast::State;
use std::fmt::Debug;


pub trait ArithmeticExpression: Debug  {
    fn evaluate(&self, state: &State) -> i32;
}

 
#[derive(Debug)]
pub struct Numeral(pub i32);

impl ArithmeticExpression for Numeral {
    fn evaluate(&self, _state: &State) -> i32 {
        self.0
    }
}
 
#[derive(Debug)]
pub struct Variable {
    pub value: String,
}

impl ArithmeticExpression for Variable {
    fn evaluate(&self, state: &State) -> i32 {
        match state.get(&self.value) {
            Some(&val) => val,
            None => panic!("Variabile '{}' non trovata nello stato!", self.value),
        }
    }
}


 
#[derive(Debug)]
pub struct Add {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}

impl ArithmeticExpression for Add {
    fn evaluate(&self, state: &State) -> i32 {
        self.left.evaluate(state) + self.right.evaluate(state)
    }
}
 
#[derive(Debug)]
pub struct Product {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}

impl ArithmeticExpression for Product {
    fn evaluate(&self, state: &State) -> i32 {
        self.left.evaluate(state) * self.right.evaluate(state)
    }
}
 
#[derive(Debug)]
pub struct Minus {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}

impl ArithmeticExpression for Minus {
    fn evaluate(&self, state: &State) -> i32 {
        self.left.evaluate(state) - self.right.evaluate(state)
    }
}

#[derive(Debug)]
pub struct Uminus{
    pub right : Box<dyn ArithmeticExpression>,
}

impl ArithmeticExpression for Uminus{
    fn evaluate(&self, state: &State) -> i32 {
        - self.right.evaluate(state)
    }
}

#[derive(Debug)]
pub struct Divide {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}

impl ArithmeticExpression for Divide {
    fn evaluate(&self, state: &State) -> i32 {
        self.left.evaluate(state) / self.right.evaluate(state)
    }
}
