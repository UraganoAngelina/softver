use crate::ast::arithmetic::ArithmeticExpression;
use crate::ast::boolean::BooleanExpression;
use crate::ast::State;
use std::fmt::Debug;

pub trait Statement: Debug {
    fn evaluate(&self, state: &mut State);
}

#[derive(Debug)]
pub struct Assign {
    pub var_name: String,
    pub expr: Box<dyn ArithmeticExpression>,
}

impl Statement for Assign {
    fn evaluate(&self, state: &mut State) {
        let value = self.expr.evaluate(state);
        state.insert(self.var_name.clone(), value);
    }
}

#[derive(Debug)]
pub struct Skip;

impl Statement for Skip {
    fn evaluate(&self, _state: &mut State) {
        // Do nothing
    }
}

#[derive(Debug)]
pub struct Concat {
    pub first: Box<dyn Statement>,
    pub second: Box<dyn Statement>,
}

impl Statement for Concat {
    fn evaluate(&self, state: &mut State) {
        self.first.evaluate(state);
        self.second.evaluate(state);
    }
}

#[derive(Debug)]
pub struct IfThenElse {
    pub guard: Box<dyn BooleanExpression>,
    pub true_expr: Box<dyn Statement>,
    pub false_expr: Box<dyn Statement>,
}

impl Statement for IfThenElse {
    fn evaluate(&self, state: &mut State) {
        if self.guard.evaluate(state) {
            self.true_expr.evaluate(state);
        } else {
            self.false_expr.evaluate(state);
        }
    }
}

#[derive(Debug)]
pub struct While {
    pub guard: Box<dyn BooleanExpression>,
    pub body: Box<dyn Statement>,
}

impl Statement for While {
    fn evaluate(&self, state: &mut State) {
        while self.guard.evaluate(state) {
            self.body.evaluate(state);
        }
    }
}
