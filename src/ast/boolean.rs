use crate::ast::arithmetic::ArithmeticExpression;
use crate::ast::State;
use std::fmt::Debug;

pub trait BooleanExpression: Debug {
    fn evaluate(&self, state: &State) -> bool;
}

#[derive(Debug)]
pub struct Boolean(pub bool);

impl BooleanExpression for Boolean {
    fn evaluate(&self, _state: &State) -> bool {
        self.0
    }
}

#[derive(Debug)]
pub struct Equal {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}

impl BooleanExpression for Equal {
    fn evaluate(&self, state: &State) -> bool {
        self.left.evaluate(state) == self.right.evaluate(state)
    }
}

#[derive(Debug)]
pub struct GreatEqual {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}

impl BooleanExpression for GreatEqual {
    fn evaluate(&self, state: &State) -> bool {
        self.left.evaluate(state) >= self.right.evaluate(state)
    }
}
#[derive(Debug)]
pub struct Great{
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}
impl BooleanExpression for Great{
    fn evaluate(&self, state: &State) -> bool {
        self.left.evaluate(state) > self.right.evaluate(state)
    }
}


#[derive(Debug)]
pub struct LessEqual {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}

impl BooleanExpression for LessEqual {
    fn evaluate(&self, state: &State) -> bool {
        self.left.evaluate(state) <= self.right.evaluate(state)
    }
}

#[derive(Debug)]
pub struct Less{
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}

impl BooleanExpression for Less{
    fn evaluate(&self, state: &State) -> bool {
        self.left.evaluate(state) < self.right.evaluate(state)   
    }
}

#[derive(Debug)]
pub struct And {
    pub left: Box<dyn BooleanExpression>,
    pub right: Box<dyn BooleanExpression>,
}

impl BooleanExpression for And {
    fn evaluate(&self, state: &State) -> bool {
        self.left.evaluate(state) && self.right.evaluate(state)
    }
}

#[derive(Debug)]
pub struct Or {
    pub left: Box<dyn BooleanExpression>,
    pub right: Box<dyn BooleanExpression>,
}

impl BooleanExpression for Or {
    fn evaluate(&self, state: &State) -> bool {
        self.left.evaluate(state) || self.right.evaluate(state)
    }
}

#[derive(Debug)]
pub struct Not {
    pub expression: Box<dyn BooleanExpression>,
}

impl BooleanExpression for Not {
    fn evaluate(&self, state: &State) -> bool {
        !(self.expression.evaluate(state))
    }
}
