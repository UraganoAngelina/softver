use crate::abstract_domain::{AbstractDomain, AbstractDomainOps};
use crate::abstract_interval::AbstractInterval;
use crate::abstract_state::AbstractState;
use crate::ast::{arithmetic::*, boolean::*, State};
use crate::{ NARROWING_FLAG, WIDENING_FLAG};
use std::fmt::Debug;
pub trait Statement: Debug {
    type Q: AbstractDomainOps + PartialEq + Clone+ Debug;
    fn clone_box(&self) -> Box<dyn Statement<Q=Self::Q>>;
    fn evaluate(&self, state: &mut State) -> State;
    fn abs_evaluate(&self, state: &mut AbstractState<Self::Q>) -> AbstractState<Self::Q>;
    fn to_string(&self) -> String;
}

#[derive(Debug)]
pub struct Assign {
    pub var_name: Box<dyn ArithmeticExpression<Q=AbstractInterval>>,
    pub expr: Box<dyn ArithmeticExpression<Q=AbstractInterval>>,
}

impl Statement for Assign {
    type Q = AbstractInterval;
    fn clone_box(&self) -> Box<dyn Statement<Q=Self::Q>> {
        Box::new(Assign {
            var_name: self.var_name.clone_box(),
            expr: self.expr.clone_box(),
        })
    }

    fn evaluate(&self, state: &mut State) -> State {
        let value = self.expr.evaluate(state);
        state.insert(self.var_name.clone_box().to_string(), value);
        state.clone()
    }
    fn abs_evaluate(&self, state: &mut AbstractState<Self::Q>) -> AbstractState<Self::Q> {
            let mut new_state = state.clone();
            let value = self.expr.abs_evaluate(&mut new_state);
            state.variables.insert(
                self.var_name.as_variable().unwrap().to_string(),
                AbstractDomain::new(value),
            );
            state.clone()
        
    }
    fn to_string(&self) -> String {
        format!("{} := {}", self.var_name.to_string(), self.expr.to_string())
    }
}

#[derive(Debug)]
pub struct Skip;

impl Statement for Skip {
    type Q = AbstractInterval;
    fn clone_box(&self) -> Box<dyn Statement<Q=Self::Q>> {
        Box::new(Skip {})
    }

    fn evaluate(&self, state: &mut State) -> State {
        state.clone()
    }
    fn abs_evaluate(&self, state: &mut AbstractState<Self::Q>) -> AbstractState<Self::Q> {
        state.clone()
    }
    fn to_string(&self) -> String {
        format!("skip")
    }
}

#[derive(Debug)]
pub struct Concat {
    pub first: Box<dyn Statement<Q=AbstractInterval>>,
    pub second: Box<dyn Statement<Q=AbstractInterval>>,
}

impl Statement for Concat {
    type Q = AbstractInterval;
    fn clone_box(&self) -> Box<dyn Statement<Q=Self::Q>> {
        Box::new(Concat {
            first: self.first.clone_box(),
            second: self.second.clone_box(),
        })
    }

    fn evaluate(&self, state: &mut State) -> State {
        let mut state_after_first = self.first.evaluate(state);
        let state_after_second = self.second.evaluate(&mut state_after_first);
        state_after_second
    }
    fn abs_evaluate(&self, state: &mut AbstractState<Self::Q>) -> AbstractState<Self::Q> {
            let new_state = self
            .second
            .abs_evaluate(&mut self.first.abs_evaluate(state));
        state.variables.extend(new_state.variables.clone());
        new_state
        
        
    }
    fn to_string(&self) -> String {
        format!("{} ; {}", self.first.to_string(), self.second.to_string())
    }
}

#[derive(Debug)]
pub struct IfThenElse {
    pub guard: Box<dyn BooleanExpression<Q=AbstractInterval>>,
    pub true_expr: Box<dyn Statement<Q=AbstractInterval>>,
    pub false_expr: Box<dyn Statement<Q=AbstractInterval>>,
}

impl Statement for IfThenElse {
    type Q = AbstractInterval;
    fn clone_box(&self) -> Box<dyn Statement<Q=Self::Q>> {
        Box::new(IfThenElse {
            guard: self.guard.clone_box(),
            true_expr: self.true_expr.clone_box(),
            false_expr: self.false_expr.clone_box(),
        })
    }

    fn evaluate(&self, state: &mut State) -> State {
        if self.guard.evaluate(state) {
            let state_after_true = self.true_expr.evaluate(state);
            state.extend(state_after_true.clone());
            state.clone()
        } else {
            let state_after_false = self.false_expr.evaluate(state);
            state.extend(state_after_false.clone());
            state.clone()
        }
    }

    fn abs_evaluate(&self, state: &mut AbstractState<Self::Q>) -> AbstractState<Self::Q> {
        let then_state = self
            .guard
            .abs_evaluate(&mut self.true_expr.abs_evaluate(state), false);
        let else_state = self
            .guard
            .abs_evaluate(&mut self.false_expr.abs_evaluate(state), false);

        let final_state = AbstractState::state_lub(&then_state, &else_state);
        state.variables.extend(final_state.variables.clone());
        final_state
    }
    fn to_string(&self) -> String {
        format!(
            "if ({}) then  {{{}}}  else {{{}}}",
            self.guard.to_string(),
            self.true_expr.to_string(),
            self.false_expr.to_string()
        )
    }
}

#[derive(Debug)]
pub struct While {
    pub guard: Box<dyn BooleanExpression<Q=AbstractInterval>>,
    pub body: Box<dyn Statement<Q=AbstractInterval>>,
}

impl Statement for While {
    type Q = AbstractInterval;
    fn clone_box(&self) -> Box<dyn Statement<Q=Self::Q>> {
        Box::new(While {
            guard: self.guard.clone_box(),
            body: self.body.clone_box(),
        })
    }

    fn evaluate(&self, state: &mut State) -> State {
        println!("WHILE INPUT STATE {:#?}", state);
        let mut prev_state: State;
        let mut current_state = state.clone();
        loop {
            prev_state = current_state.clone();
            if self.guard.evaluate(&mut current_state) {
                current_state = self.body.evaluate(&mut current_state);
            } else {
            }
            if current_state == prev_state {
                break;
            }
        }
        //fix-point found now return the state
        state.extend(current_state.clone());
        println!("state after while evaluation {:#?}", state);
        current_state
    }

    fn abs_evaluate(&self, state: &mut AbstractState<Self::Q>) -> AbstractState<Self::Q> {
        let wid = WIDENING_FLAG
            .lock()
            .expect("failed to read widening flag in while loop");
        let narrow = NARROWING_FLAG
            .lock()
            .expect("failed to read narrowing flag in while loop");
        let precondition = state.clone();
        println!("PRECONDITION: {}", precondition);
        let mut _guard_result = AbstractState::new();
        let mut _body_result = AbstractState::new();
        let mut _prev_state = state.clone();
        let mut current_state = state.clone();
        loop {
            _guard_result = self.guard.abs_evaluate(&mut current_state.clone(), false);
            _body_result = self.body.abs_evaluate(&mut _guard_result.clone());
            _body_result = _prev_state.state_lub(&_body_result.clone());
            if *wid == true {
                current_state = _prev_state.state_widening(&_body_result.clone());
            }
            // Fixpoint check
            if current_state.clone() == _prev_state.clone() {
                break;
            }
            _prev_state = current_state.clone();
        }
        println!("CYCLE INVARIANT: {}", current_state);
        _prev_state = precondition.clone();
        if *narrow == true {
            loop {
                _guard_result = self.guard.abs_evaluate(&mut current_state.clone(), false);
                _body_result = self.body.abs_evaluate(&mut _guard_result.clone());
                _body_result = _prev_state.state_lub(&_body_result.clone());
                current_state = current_state.clone().state_narrowing(&_body_result.clone());
                if current_state.clone() == _prev_state.clone() {
                    break;
                }
                _prev_state = current_state.clone();
            }
        }
        // filtering with !guard
        let postcondition = self.guard.abs_evaluate(&mut current_state.clone(), true);
        state.variables.extend(postcondition.variables.clone());
        println!("CYCLE POSTCONDITION: {}", postcondition);
        postcondition
    }

    fn to_string(&self) -> String {
        format!(
            "while ({}) {{{}}} ",
            self.guard.to_string(),
            self.body.to_string()
        )
    }
}

#[derive(Debug)]
pub struct For {
    pub init: Box<dyn Statement<Q=AbstractInterval>>,
    pub guard: Box<dyn BooleanExpression<Q=AbstractInterval>>,
    pub increment: Box<dyn ArithmeticExpression<Q=AbstractInterval>>,
    pub body: Box<dyn Statement<Q=AbstractInterval>>,
}

impl Statement for For {
    type Q = AbstractInterval;
    fn clone_box(&self) -> Box<dyn Statement<Q=Self::Q>> {
        Box::new(For {
            init: self.init.clone_box(),
            guard: self.guard.clone_box(),
            increment: self.increment.clone_box(),
            body: self.body.clone_box(),
        })
    }

    //for loop evaluation
    fn evaluate(&self, state: &mut State) -> State {
        println!("FOR INPUT STATE {:#?}", state);
        let mut prev_state: State;
        let mut current_state = state.clone();
        current_state = self.init.evaluate(&mut current_state);
        loop {
            prev_state = current_state.clone();
            if self.guard.evaluate(&mut current_state) {
                current_state = self.body.evaluate(&mut current_state);
                let _ = self.increment.evaluate(&mut current_state);
            } else {
            }
            if current_state == prev_state {
                break;
            }
        }
        state.extend(current_state.clone());
        println!("state after for evaluation {:#?}", state);
        current_state
    }

    fn abs_evaluate(&self, state: &mut AbstractState<Self::Q>) -> AbstractState<Self::Q> {
        let wid = WIDENING_FLAG
            .lock()
            .expect("failed to read widening flag in for loop");
        let narrow = NARROWING_FLAG
            .lock()
            .expect("failed to read narrowing flag in for loop");
        let precondition = state.clone();
        println!("PRECONDITION {}", precondition);
        self.init.abs_evaluate(&mut state.clone());
        let mut _guard_result = AbstractState::new();
        let mut _body_result = AbstractState::new();
        let mut _prev_state = state.clone();
        let mut current_state = state.clone();
        let mut _increment_result = AbstractInterval::new(0 as i64, 0 as i64);
        loop {
            _prev_state = current_state.clone();

            _guard_result = self.guard.abs_evaluate(&mut _prev_state.clone(), false);

            _body_result = self.body.abs_evaluate(&mut _guard_result.clone());
            _increment_result = self.increment.abs_evaluate(&mut _body_result);

            _body_result = _prev_state.state_lub(&_body_result.clone());

            if *wid == true {
                current_state = _prev_state.state_widening(&_body_result.clone());
            }

            if current_state == _prev_state {
                break;
            }
        }
        println!("CYCLE INVARIANT {}", current_state);
        _prev_state = precondition.clone();
        
            loop {
                _guard_result = self.guard.abs_evaluate(&mut current_state.clone(), false);
                _body_result = self.body.abs_evaluate(&mut _guard_result.clone());
                _body_result = _prev_state.state_lub(&_body_result.clone());
                _increment_result = self.increment.abs_evaluate(&mut _body_result);
                if *narrow == true {current_state = current_state.state_narrowing(&_body_result.clone());}
                if current_state == _prev_state {
                    break;
                }
                _prev_state = current_state.clone();
            }
        
        // filtering with !guard
        let postcondition = self.guard.abs_evaluate(&mut current_state.clone(), true);
        println!("CYCLE POSTCONDITION: {}", postcondition);
        state.clone()
    }

    fn to_string(&self) -> String {
        format!(
            "for ({} ; {} ; {}) {{{}}} ",
            self.init.to_string(),
            self.guard.to_string(),
            self.increment.to_string(),
            self.body.to_string()
        )
    }
}

#[derive(Debug)]
pub struct RepeatUntil {
    pub body: Box<dyn Statement<Q=AbstractInterval>>,
    pub guard: Box<dyn BooleanExpression<Q=AbstractInterval>>,
}

impl Statement for RepeatUntil {
    type Q = AbstractInterval;
    fn clone_box(&self) -> Box<dyn Statement<Q=Self::Q>> {
        Box::new(RepeatUntil {
            body: self.body.clone_box(),
            guard: self.guard.clone_box(),
        })
    }

    //Repeat until evaluation
    fn evaluate(&self, state: &mut State) -> State {
        println!("REPEAT UNTIL INPUT STATE {:#?}", state);
        let mut prev_state: State;
        //One body executione guaranteed
        let mut current_state = self.body.evaluate(&mut state.clone());
        loop {
            prev_state = current_state.clone();
            if !self.guard.evaluate(&mut current_state) {
                current_state = self.body.evaluate(&mut current_state);
            } else {
            }
            if current_state == prev_state {
                break;
            }
        }
        println!("state after repeat until evaluation {:#?}", state);
        current_state
    }

    fn abs_evaluate(&self, state: &mut AbstractState<Self::Q>) -> AbstractState<Self::Q> {
        let wid = WIDENING_FLAG
            .lock()
            .expect("failed to read widening flag in repeat until loop");
        let narrow = NARROWING_FLAG
            .lock()
            .expect("failed to read narrowing flag in repeat until loop");
        let precondition = state.clone();
        println!("PRECONDITION {}", precondition);
        let mut _guard_result = AbstractState::new();
        let mut _body_result = AbstractState::new();
        let mut prev_state = state.clone();
        let mut current_state = self.body.abs_evaluate(&mut prev_state);
        loop {
            prev_state = current_state.clone();
            _guard_result = self.guard.abs_evaluate(&mut prev_state.clone(), false);
            _body_result = self.body.abs_evaluate(&mut _guard_result.clone());
            _body_result = prev_state.state_lub(&_body_result.clone());
            if *wid == true {
                current_state = prev_state.state_widening(&_body_result.clone());
            }
            //fixpoint check
            if current_state == prev_state {
                break;
            }
        }
        println!("CYCLE INVARIANT: {}", current_state);
        prev_state = precondition.clone();
        if *narrow == true {
            loop {
                _guard_result = self.guard.abs_evaluate(&mut current_state.clone(), false);
                _body_result = self.body.abs_evaluate(&mut _guard_result.clone());
                _body_result = prev_state.state_lub(&_body_result.clone());
                current_state = current_state.state_narrowing(&_body_result.clone());
                if current_state == prev_state {
                    break;
                }
                prev_state = current_state.clone();
            }
        }
        // filtering with !guard
        let postcondition = self.guard.abs_evaluate(&mut current_state.clone(), true);
        state.variables.extend(postcondition.variables.clone());
        println!("CYCLE POSTCONDITION: {}", postcondition);
        state.clone()
    }

    fn to_string(&self) -> String {
        format!(
            "repeat {{{}}} until ({}) ",
            self.body.to_string(),
            self.guard.to_string()
        )
    }
}
