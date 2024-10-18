use crate::ast::arithmetic::ArithmeticExpression;
use crate::ast::boolean::BooleanExpression;
use crate::ast::State;
use std::fmt::Debug;

pub trait Statement: Debug {
    fn clone_box(&self) -> Box<dyn Statement>;
    fn evaluate(&self, state: &mut State);
    fn as_any(&self) -> &dyn std::any::Any;
    
}

#[derive(Debug)]
pub struct Assign {
    pub var_name: String,
    pub expr: Box<dyn ArithmeticExpression>,
}

impl Statement for Assign {
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(Assign {
            var_name: self.var_name.clone(), // Clona il nome della variabile
            expr: self.expr.clone_box(),     // Clona l'espressione aritmetica
        })
    }
    fn evaluate(&self, state: &mut State) {
        let value = self.expr.evaluate(state);
        state.insert(self.var_name.clone(), value);
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Debug)]
pub struct Skip;

impl Statement for Skip {
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(Skip {})
    }
    fn evaluate(&self, _state: &mut State) {
        // Do nothing
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Debug)]
pub struct Concat {
    pub first: Box<dyn Statement>,
    pub second: Box<dyn Statement>,
}

impl Statement for Concat {
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(Concat {
            first: self.first.clone_box(),
            second: self.second.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) {
        self.first.evaluate(state);
        self.second.evaluate(state);
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Debug)]
pub struct IfThenElse {
    pub guard: Box<dyn BooleanExpression>,
    pub true_expr: Box<dyn Statement>,
    pub false_expr: Box<dyn Statement>,
}

impl Statement for IfThenElse {
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(IfThenElse {
            guard: self.guard.clone_box(),
            true_expr: self.true_expr.clone_box(),
            false_expr: self.false_expr.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) {
        if self.guard.evaluate(state) {
            self.true_expr.evaluate(state);
        } else {
            self.false_expr.evaluate(state);
        }
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Debug)]
pub struct While {
    pub guard: Box<dyn BooleanExpression>,
    pub body: Box<dyn Statement>,
}

impl Statement for While {
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(While {
            guard: self.guard.clone_box(),
            body: self.body.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) {
        while self.guard.evaluate(state) {
            self.body.evaluate(state);
        }
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Debug)]
pub struct For {
    pub init: Box<dyn Statement>,
    pub guard: Box<dyn BooleanExpression>,
    pub increment: Box<dyn Statement>,
    pub body: Box<dyn Statement>,
}
impl Statement for For {
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(For {
            init: self.init.clone_box(),
            guard: self.guard.clone_box(),
            increment: self.increment.clone_box(),
            body: self.body.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) {
        todo!()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Debug)]
pub struct RepeatUntil{
    pub body: Box<dyn Statement>,
    pub guard: Box<dyn BooleanExpression>,
}
impl Statement for RepeatUntil{
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(RepeatUntil{
            body: self.body.clone_box(),
            guard: self.guard.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) {
        todo!()
    }

}
