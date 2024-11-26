use crate::ast::State;
use crate::ast::{arithmetic::*, boolean::*};
use std::fmt::Debug;


pub trait Statement: Debug {
    fn clone_box(&self) -> Box<dyn Statement>;
    fn evaluate(&self, state: &mut State) -> State;
}

#[derive(Debug)]
pub struct Assign {
    pub var_name: Box<dyn ArithmeticExpression>,
    pub expr: Box<dyn ArithmeticExpression>,
}

impl Statement for Assign {
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(Assign {
            var_name: self.var_name.clone_box(),
            expr: self.expr.clone_box(),
        })
    }

    fn evaluate(&self, state: &mut State) -> State {
        let value = self.expr.evaluate(state);
        println!(
            "value in assign eval {:?}, for var {:?}",
            value,
            self.var_name.clone_box()
        );
        state.insert(self.var_name.clone_box().to_string(), value);
        println!("state after assign insertion: {:?}", state);
        state.clone() 
    }
}

#[derive(Debug)]
pub struct Skip;

impl Statement for Skip {
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(Skip {})
    }

    fn evaluate(&self, state: &mut State) -> State {
        state.clone() 
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

    fn evaluate(&self, state: &mut State) -> State {
        let mut state_after_first = self.first.evaluate(state);
        println!("state after first {:?}", state_after_first);
        let state_after_second = self.second.evaluate(&mut state_after_first); 
        println!("state after second {:?}", state_after_second);
        //state.clear();
        state.extend(state_after_second.clone());

        state_after_second 
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

    fn evaluate(&self, state: &mut State) -> State {
        if self.guard.evaluate(state) {
            let state_after_true = self.true_expr.evaluate(state);
            println!("state after true: {:?}", state_after_true);
            state.extend(state_after_true.clone());
            println!("extended state: {:?}", state);
            state_after_true
        } else {
            let state_after_false = self.false_expr.evaluate(state); 
            println!("state after false: {:?}", state_after_false); 
            state.extend(state_after_false.clone());
            println!("extended state: {:?}", state);
            state_after_false
        }
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

    fn evaluate(&self, state: &mut State) -> State {
        let mut current_state = state.clone();
        while self.guard.evaluate(& mut current_state) {
            current_state = self.body.evaluate(&mut current_state);
            println!("while body current state: {:?}", current_state);
        }
        state.extend(current_state.clone());
        println!("Extended state after while eval: {:?}", state);
        state.clone()
    }
}

#[derive(Debug)]
pub struct For {
    pub init: Box<dyn Statement>,
    pub guard: Box<dyn BooleanExpression>,
    pub increment: Box<dyn ArithmeticExpression>,
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

    fn evaluate(&self, state: &mut State) -> State{
        println!("State before init: {:?}", state);
        self.init.evaluate(state);
        println!("State after init: {:?}", state);

        while self.guard.evaluate(state) {
            println!("State before body: {:?}", state);
            self.body.evaluate(state);
            println!("State after body: {:?}", state);

            let _ = self.increment.evaluate(state); 
            println!("State after increment: {:?}", state);
        }
        println!("Final state after for eval: {:?}", state);
        state.clone()
    }
}


#[derive(Debug)]
pub struct RepeatUntil {
    pub body: Box<dyn Statement>,
    pub guard: Box<dyn BooleanExpression>,
}

impl Statement for RepeatUntil {
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(RepeatUntil {
            body: self.body.clone_box(),
            guard: self.guard.clone_box(),
        })
    }

    fn evaluate(&self, state: &mut State) -> State {
        let mut current_state = state.clone();
        loop {
            current_state = self.body.evaluate(&mut current_state);
            println!("repeat until body current state: {:?}", current_state);
            if self.guard.evaluate(& mut current_state) {
                break;
            }
        }
        state.extend(current_state.clone());
        println!("Extended state after repeat until eval: {:?}", state);
        current_state
    }
}

