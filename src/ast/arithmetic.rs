use crate::abstract_domain::AbstractInterval;
use crate::abstract_state::AbstractState;
use crate::ast::State;
use std::any::Any;
use std::fmt::Debug;

pub trait ArithmeticExpression: Debug {
    fn clone_box(&self) -> Box<dyn ArithmeticExpression>;
    fn as_variable(&self) -> Option<&Variable>;
    fn evaluate(&self, state: &mut State) -> i64;
    fn as_any(&self) -> &dyn Any;

    fn to_string(&self) -> String;
    fn abs_evaluate(&self, abs_state: &mut AbstractState) -> AbstractInterval<i64>;
}

#[derive(Debug)]
pub struct Numeral(pub i64);

impl ArithmeticExpression for Numeral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ArithmeticExpression> {
        Box::new(Numeral(self.0))
    }
    fn as_variable(&self) -> Option<&Variable> {
        None
    }
    fn evaluate(&self, _state: &mut State) -> i64 {
        self.0
    }
    fn to_string(&self) -> String {
        self.0.to_string()
    }
    fn abs_evaluate(&self, _abs_state: &mut AbstractState) -> AbstractInterval<i64> {
        AbstractInterval::new(self.0, self.0)
    }
}

#[derive(Debug)]
pub struct Variable {
    pub value: String,
}

impl ArithmeticExpression for Variable {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ArithmeticExpression> {
        Box::new(Variable {
            value: self.value.clone(),
        })
    }
    fn as_variable(&self) -> Option<&Variable> {
        Some(self)
    }
    fn evaluate(&self, state: &mut State) -> i64 {
        println!("searching for variable {}" , self.value);
        println!("state situation {:#?}", state);
        *state
            .get(&self.value)
            .expect("Variable  not found in the state!")
    }
    fn to_string(&self) -> String {
        self.value.clone()
    }
    fn abs_evaluate(&self, abs_state: &mut AbstractState) -> AbstractInterval<i64> {
        println!("searching for variable {}" , self.value);
        *abs_state
            .variables
            .get(&self.value)
            .expect("Variable not found in the abstract state!")
    }
}

#[derive(Debug)]
pub struct Add {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}

impl ArithmeticExpression for Add {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ArithmeticExpression> {
        Box::new(Add {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }
    fn as_variable(&self) -> Option<&Variable> {
        None
    }
    fn evaluate(&self, state: &mut State) -> i64 {
        self.left.evaluate(state) + self.right.evaluate(state)
    }
    fn to_string(&self) -> String {
        format!("({} + {})", self.left.to_string(), self.right.to_string())
    }
    fn abs_evaluate(&self, abs_state: &mut AbstractState) -> AbstractInterval<i64> {
        self.left.abs_evaluate(abs_state) + self.right.abs_evaluate(abs_state)
    }
}

#[derive(Debug)]
pub struct Product {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}

impl ArithmeticExpression for Product {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ArithmeticExpression> {
        Box::new(Product {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }
    fn as_variable(&self) -> Option<&Variable> {
        None
    }
    fn evaluate(&self, state: &mut State) -> i64 {
        self.left.evaluate(state) * self.right.evaluate(state)
    }
    fn to_string(&self) -> String {
        format!("({} * {})", self.left.to_string(), self.right.to_string())
    }
    fn abs_evaluate(&self, abs_state: &mut AbstractState) -> AbstractInterval<i64> {
        self.left.abs_evaluate(abs_state) * self.right.abs_evaluate(abs_state)
    }
}

#[derive(Debug)]
pub struct Minus {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}

impl ArithmeticExpression for Minus {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ArithmeticExpression> {
        Box::new(Minus {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }
    fn as_variable(&self) -> Option<&Variable> {
        None
    }
    fn evaluate(&self, state: &mut State) -> i64 {
        self.left.evaluate(state) - self.right.evaluate(state)
    }
    fn to_string(&self) -> String {
        format!("({} - {})", self.left.to_string(), self.right.to_string())
    }
    fn abs_evaluate(&self, abs_state: &mut AbstractState) -> AbstractInterval<i64> {
        self.left.abs_evaluate(abs_state) - self.right.abs_evaluate(abs_state)
    }
}

#[derive(Debug)]
pub struct Uminus {
    pub right: Box<dyn ArithmeticExpression>,
}
impl ArithmeticExpression for Uminus {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ArithmeticExpression> {
        Box::new(Uminus {
            right: self.right.clone_box(),
        })
    }
    fn as_variable(&self) -> Option<&Variable> {
        None
    }
    fn evaluate(&self, state: &mut State) -> i64 {
        -self.right.evaluate(state)
    }
    fn to_string(&self) -> String {
        format!("-{}", self.right.to_string())
    }
    fn abs_evaluate(&self, abs_state: &mut AbstractState) -> AbstractInterval<i64> {
        -self.right.abs_evaluate(abs_state)
    }
}

#[derive(Debug)]
pub struct Divide {
    pub left: Box<dyn ArithmeticExpression>,
    pub right: Box<dyn ArithmeticExpression>,
}
impl ArithmeticExpression for Divide {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ArithmeticExpression> {
        Box::new(Divide {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }
    fn as_variable(&self) -> Option<&Variable> {
        None
    }
    fn evaluate(&self, state: &mut State) -> i64 {
        if self.right.evaluate(state) != 0 {
            self.left.evaluate(state) / self.right.evaluate(state)
        } else {
            unreachable!(
                "**RUNTIME ERROR, division by zero found while applying denotational semantics**"
            )
        }
    }
    fn to_string(&self) -> String {
        format!("({} / {})", self.left.to_string(), self.right.to_string())
    }
    fn abs_evaluate(&self, abs_state: &mut AbstractState) -> AbstractInterval<i64> {
        let zero_int = AbstractInterval::new(0, 0);
        if self.right.abs_evaluate(abs_state) != zero_int {
            self.left.abs_evaluate(abs_state) / self.right.abs_evaluate(abs_state)
        } else {
            unreachable!("**RUNTIME ERROR, division by zero found while interpreting**")
        }
    }
}

#[derive(Debug)]
pub struct PlusPlus {
    pub var: Box<dyn ArithmeticExpression>,
}
impl ArithmeticExpression for PlusPlus {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ArithmeticExpression> {
        Box::new(PlusPlus {
            var: self.var.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> i64 {
        //Variable evaluation -> i64
        let mut value = self.var.evaluate(state);
        value += 1;
        //changes the state (like a statement)
        state.insert(self.var.clone_box().to_string(), value);
        // but returns an integer value
        value
    }
    
    fn as_variable(&self) -> Option<&Variable> {
        self.var.as_variable()
    }
    fn to_string(&self) -> String {
        format!("{}++", self.var.to_string())
    }
    fn abs_evaluate(&self, abs_state: &mut AbstractState) -> AbstractInterval<i64> {
        let value = self.var.abs_evaluate(abs_state);
        match value {
            AbstractInterval::Bottom => AbstractInterval::Bottom,
            AbstractInterval::Top => AbstractInterval::Top,
            AbstractInterval::Bounded { lower, upper } => {
                if upper < i64::max_value() {
                    if lower > i64::min_value() {
                        let newupper = upper + 1;
                        let new_interval = AbstractInterval::new(lower, newupper);
                        abs_state
                            .variables
                            .insert(self.var.to_string(), new_interval);
                        new_interval
                    } else {
                        AbstractInterval::Top
                    }
                } else {
                    AbstractInterval::Top
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct MinusMinus {
    pub var: Box<dyn ArithmeticExpression>,
}
impl ArithmeticExpression for MinusMinus {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ArithmeticExpression> {
        Box::new(MinusMinus {
            var: self.var.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> i64 {
        //Variable evaluation -> i64
        let mut value = self.var.evaluate(state);
        println!(
            "value in assign eval {:?}, for var {:?}",
            value,
            self.var.clone_box()
        );
        value -= 1;
        state.insert(self.var.clone_box().to_string(), value);
        println!("state after plus plus evaluation: {:?}", state);
        value
    }
    fn as_variable(&self) -> Option<&Variable> {
        self.var.as_variable()
    }
    fn to_string(&self) -> String {
        format!("{}--", self.var.to_string())
    }
    fn abs_evaluate(&self, abs_state: &mut AbstractState) -> AbstractInterval<i64> {
        let value = self.var.abs_evaluate(abs_state);
        match value {
            AbstractInterval::Bottom => AbstractInterval::Bottom,
            AbstractInterval::Top => AbstractInterval::Top,
            AbstractInterval::Bounded { lower, upper } => {
                if upper < i64::max_value() {
                    if lower > i64::min_value() {
                        let newlower = lower - 1;
                        let new_interval = AbstractInterval::new(newlower, upper);
                        abs_state
                            .variables
                            .insert(self.var.to_string(), new_interval);
                        new_interval
                    }
                    else {
                        AbstractInterval::Top
                    }
                }
                else {
                    AbstractInterval::Top
                }
            }
        }
    }
}
