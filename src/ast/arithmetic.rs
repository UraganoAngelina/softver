use crate::abstract_domain::{AbstractDomain, AbstractDomainOps};
use crate::abstract_interval::AbstractInterval;
use crate::abstract_state::AbstractState;
use crate::ast::State;
use crate::CONSTANTS_VECTOR;
use crate::{M, N};
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;

use super::{Node, Op};

pub trait ArithmeticExpression: Debug {
    type Q: AbstractDomainOps + PartialEq + Clone + Debug;
    fn clone_box(&self) -> Box<dyn ArithmeticExpression<Q = Self::Q>>;
    fn as_variable(&self) -> Option<&Variable>;
    fn evaluate(&self, state: &mut State) -> i64;
    fn as_any(&self) -> &dyn Any;

    fn to_string(&self) -> String;
    fn abs_evaluate(&self, abs_state: &mut AbstractState<Self::Q>) -> AbstractInterval;
    fn extract_variables(&self) -> Vec<&Variable>;
    fn to_ast(&self, abs_state: &mut AbstractState<Self::Q>, var_leaves: &mut HashMap<String,AbstractInterval >) -> Node;
}

#[derive(Debug)]
pub struct Numeral(pub i64);

impl ArithmeticExpression for Numeral {
    type Q = AbstractInterval;
    fn to_ast(&self, _abs_state: &mut AbstractState<Self::Q>, _var_leaves : &mut HashMap<String, AbstractInterval>) -> Node {
        // Creo un nodo Internal con operatore Sub e due sotto-nodi
        Node::ConstantLeaf(AbstractInterval::Bounded {
            lower: self.0,
            upper: self.0,
        })
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ArithmeticExpression<Q = Self::Q>> {
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
    fn abs_evaluate(&self, _abs_state: &mut AbstractState<Self::Q>) -> AbstractInterval {
        let mut constant = CONSTANTS_VECTOR
            .lock()
            .expect("FAILED TO LOCK THE CONSTANT VECTOR");
        //put the constant in the dedicated vector for threshold widening
        if ! constant.contains(&self.0){
            constant.push(self.0.clone());
        }
        //return a constant interval
        AbstractInterval::new(self.0, self.0)
    }
    fn extract_variables(&self) -> Vec<&Variable> {
        Vec::new()
    }
}

#[derive(Debug)]
pub struct Variable {
    pub value: String,
}

impl ArithmeticExpression for Variable {
    type Q = AbstractInterval;
    fn to_ast(&self, abs_state: &mut AbstractState<Self::Q> , var_leaves : &mut HashMap<String, AbstractInterval>) -> Node {
        let value = self.abs_evaluate(abs_state);
        var_leaves.insert(self.value.clone(), value);
        Node::VarLeaf(self.value.clone(), value)
    }
    fn evaluate(&self, state: &mut State) -> i64 {
        *state
            .get(&self.value)
            .expect("Variable  not found in the state!")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ArithmeticExpression<Q = Self::Q>> {
        Box::new(Variable {
            value: self.value.clone(),
        })
    }
    fn as_variable(&self) -> Option<&Variable> {
        Some(self)
    }

    fn to_string(&self) -> String {
        self.value.clone()
    }
    fn abs_evaluate(&self, abs_state: &mut AbstractState<Self::Q>) -> AbstractInterval {
        // println!("STATE SITUATION {}", abs_state);
        // println!("SEARCH FOR {}", self.value);
        let res = *abs_state
            .variables
            .get(&self.value)
            .expect("Variable not found in the abstract state!");
        // println!("VALUE {}", res.value);
        return res.value;
    }
    fn extract_variables(&self) -> Vec<&Variable> {
        let mut vars = Vec::new();
        // Se il lato sinistro è una variabile o contiene variabili
        if let Some(v) = self.as_variable() {
            vars.push(v);
        }
        vars
    }
}

#[derive(Debug)]
pub struct Add {
    pub left: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
    pub right: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
}

impl ArithmeticExpression for Add {
    type Q = AbstractInterval;
    fn to_ast(&self, abs_state: &mut AbstractState<Self::Q>, var_leaves : &mut HashMap<String, AbstractInterval>) -> Node {
        // Creo un nodo Internal con operatore Sub e due sotto-nodi
        Node::Internal(
            Op::Add,
            self.abs_evaluate(abs_state),
            Box::new(self.left.to_ast(abs_state , var_leaves)),
            Box::new(self.right.to_ast(abs_state, var_leaves)),
        )
    }
    fn evaluate(&self, state: &mut State) -> i64 {
        self.left.evaluate(state) + self.right.evaluate(state)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ArithmeticExpression<Q = Self::Q>> {
        Box::new(Add {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }
    fn as_variable(&self) -> Option<&Variable> {
        None
    }
    fn to_string(&self) -> String {
        format!("({} + {})", self.left.to_string(), self.right.to_string())
    }
    fn abs_evaluate(&self, abs_state: &mut AbstractState<Self::Q>) -> AbstractInterval {
        let result =self.left.abs_evaluate(abs_state) + self.right.abs_evaluate(abs_state);
        // println!("add result {}", result);
        result
    }
    fn extract_variables(&self) -> Vec<&Variable> {
        let mut vars = Vec::new();
        // Se il lato sinistro è una variabile o contiene variabili
        if let Some(v) = self.left.as_variable() {
            vars.push(v);
        } else {
            // Se non è una variabile diretta, prova a estrarle ricorsivamente
            vars.extend(self.left.extract_variables());
        }
        // Stessa logica per il lato destro
        if let Some(v) = self.right.as_variable() {
            vars.push(v);
        } else {
            vars.extend(self.right.extract_variables());
        }
        vars
    }
}

#[derive(Debug)]
pub struct Product {
    pub left: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
    pub right: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
}

impl ArithmeticExpression for Product {
    type Q = AbstractInterval;
    fn to_ast(&self, abs_state: &mut AbstractState<Self::Q> , var_leaves : &mut HashMap<String, AbstractInterval>) -> Node {
        // Creo un nodo Internal con operatore Sub e due sotto-nodi
        Node::Internal(
            Op::Mul,
            self.abs_evaluate(abs_state),
            Box::new(self.left.to_ast(abs_state, var_leaves)),
            Box::new(self.right.to_ast(abs_state, var_leaves)),
        )
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ArithmeticExpression<Q = Self::Q>> {
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
    fn abs_evaluate(&self, abs_state: &mut AbstractState<Self::Q>) -> AbstractInterval {
        self.left.abs_evaluate(abs_state) * self.right.abs_evaluate(abs_state)
    }
    fn extract_variables(&self) -> Vec<&Variable> {
        let mut vars = Vec::new();
        // Se il lato sinistro è una variabile o contiene variabili
        if let Some(v) = self.left.as_variable() {
            vars.push(v);
        } else {
            // Se non è una variabile diretta, prova a estrarle ricorsivamente
            vars.extend(self.left.extract_variables());
        }
        // Stessa logica per il lato destro
        if let Some(v) = self.right.as_variable() {
            vars.push(v);
        } else {
            vars.extend(self.right.extract_variables());
        }
        vars
    }
}

#[derive(Debug)]
pub struct Minus {
    pub left: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
    pub right: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
}

impl ArithmeticExpression for Minus {
    type Q = AbstractInterval;
    fn to_ast(&self, abs_state: &mut AbstractState<Self::Q>, var_leaves : &mut HashMap<String, AbstractInterval>) -> Node {
        // Creo un nodo Internal con operatore Sub e due sotto-nodi
        Node::Internal(
            Op::Sub,
            self.abs_evaluate(abs_state),
            Box::new(self.left.to_ast(abs_state, var_leaves)),
            Box::new(self.right.to_ast(abs_state, var_leaves)),
        )
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ArithmeticExpression<Q = Self::Q>> {
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
    fn abs_evaluate(&self, abs_state: &mut AbstractState<Self::Q>) -> AbstractInterval {
        let lhs = self.left.abs_evaluate(abs_state);
        let rhs =self.right.abs_evaluate(abs_state);
        let result =lhs - rhs;
        // println!("{} - {} sub result {}",lhs, rhs, result);
        result
    }
    fn extract_variables(&self) -> Vec<&Variable> {
        let mut vars = Vec::new();
        // Se il lato sinistro è una variabile o contiene variabili
        if let Some(v) = self.left.as_variable() {
            vars.push(v);
        } else {
            // Se non è una variabile diretta, prova a estrarle ricorsivamente
            vars.extend(self.left.extract_variables());
        }
        // Stessa logica per il lato destro
        if let Some(v) = self.right.as_variable() {
            vars.push(v);
        } else {
            vars.extend(self.right.extract_variables());
        }
        vars
    }
}

#[derive(Debug)]
pub struct Uminus {
    pub right: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
}
impl ArithmeticExpression for Uminus {
    type Q = AbstractInterval;
    fn to_ast(&self, abs_state: &mut AbstractState<Self::Q>, var_leaves : &mut HashMap<String, AbstractInterval>) -> Node {
        let sub_tree = Box::new(self.right.to_ast(abs_state, var_leaves));
        Node::UInternal(Op::Uminus, -self.abs_evaluate(abs_state), sub_tree)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ArithmeticExpression<Q = Self::Q>> {
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
    fn abs_evaluate(&self, abs_state: &mut AbstractState<Self::Q>) -> AbstractInterval {
        -self.right.abs_evaluate(abs_state)
    }
    fn extract_variables(&self) -> Vec<&Variable> {
        let mut vars = Vec::new();
        // Se il lato sinistro è una variabile o contiene variabili
        if let Some(v) = self.right.as_variable() {
            vars.push(v);
        } else {
            // Se non è una variabile diretta, prova a estrarle ricorsivamente
            vars.extend(self.right.extract_variables());
        }
        vars
    }
}

#[derive(Debug)]
pub struct Divide {
    pub left: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
    pub right: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
}
impl ArithmeticExpression for Divide {
    type Q = AbstractInterval;
    fn to_ast(&self, abs_state: &mut AbstractState<Self::Q>, var_leaves : &mut HashMap<String, AbstractInterval>) -> Node {
        Node::Internal(
            Op::Div,
            self.abs_evaluate(abs_state),
            Box::new(self.left.to_ast(abs_state, var_leaves)),
            Box::new(self.right.to_ast(abs_state, var_leaves)),
        )
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ArithmeticExpression<Q = Self::Q>> {
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
    fn abs_evaluate(&self, abs_state: &mut AbstractState<Self::Q>) -> AbstractInterval {
        let zero_int = AbstractInterval::new(0, 0);
        if self.right.abs_evaluate(abs_state) != zero_int {
            self.left.abs_evaluate(abs_state) / self.right.abs_evaluate(abs_state)
        } else {
            unreachable!("**RUNTIME ERROR, division by zero found while interpreting**")
        }
    }
    fn extract_variables(&self) -> Vec<&Variable> {
        let mut vars = Vec::new();
        // Se il lato sinistro è una variabile o contiene variabili
        if let Some(v) = self.left.as_variable() {
            vars.push(v);
        } else {
            // Se non è una variabile diretta, prova a estrarle ricorsivamente
            vars.extend(self.left.extract_variables());
        }
        // Stessa logica per il lato destro
        if let Some(v) = self.right.as_variable() {
            vars.push(v);
        } else {
            vars.extend(self.right.extract_variables());
        }
        vars
    }
}

#[derive(Debug)]
pub struct PlusPlus {
    pub var: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
}
impl ArithmeticExpression for PlusPlus {
    fn to_ast(&self, abs_state: &mut AbstractState<Self::Q>, var_leaves : &mut HashMap<String, AbstractInterval>) -> Node {
        Node::Internal(
            Op::Add,
            self.abs_evaluate(abs_state),
            Box::new(self.var.to_ast(abs_state, var_leaves)),
            Box::new((Numeral(1)).to_ast(abs_state, var_leaves)),
        )
    }
    type Q = AbstractInterval;
    fn evaluate(&self, state: &mut State) -> i64 {
        //Variable evaluation -> i64
        let mut value = self.var.evaluate(state);
        value += 1;
        //changes the state (like a statement)
        state.insert(self.var.clone_box().to_string(), value);
        // but returns an integer value
        value
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ArithmeticExpression<Q = Self::Q>> {
        Box::new(PlusPlus {
            var: self.var.clone_box(),
        })
    }

    fn as_variable(&self) -> Option<&Variable> {
        self.var.as_variable()
    }
    fn to_string(&self) -> String {
        format!("{}++", self.var.to_string())
    }
    fn abs_evaluate(&self, abs_state: &mut AbstractState<Self::Q>) -> AbstractInterval {
        let m = *M.lock().unwrap();
        let n = *N.lock().unwrap();
        let value = self.var.abs_evaluate(abs_state);
        match value {
            AbstractInterval::Bottom => AbstractInterval::Bottom,
            AbstractInterval::Top => AbstractInterval::Top,
            AbstractInterval::Bounded { lower, upper } => {
                if upper < n {
                    if lower > m {
                        let newupper = upper + 1;
                        let new_interval = AbstractInterval::new(lower, newupper);
                        let new_value = AbstractDomain::new(new_interval);
                        abs_state.variables.insert(self.var.to_string(), new_value);
                        //println!("state print in plus plus {}", abs_state);
                        new_interval
                    } else {
                        let new_int = AbstractInterval::Bounded { lower: m, upper };
                        // abs_state
                        //     .variables
                        //     .insert(self.var.to_string(), abstract_domain::AbstractDomain { value: new_int  });
                        abs_state.update_interval(self.var.to_string().as_str(), new_int);
                        new_int
                    }
                } else {
                    let new_int = AbstractInterval::Bounded { lower, upper: n };
                    // abs_state
                    //         .variables
                    //         .insert(self.var.to_string(), abstract_domain::AbstractDomain { value: new_int });
                    abs_state.update_interval(self.var.to_string().as_str(), new_int);
                    new_int
                }
            }
        }
    }
    fn extract_variables(&self) -> Vec<&Variable> {
        let mut vars = Vec::new();
        // Se il lato sinistro è una variabile o contiene variabili
        if let Some(v) = self.var.as_variable() {
            vars.push(v);
        } else {
            // Se non è una variabile diretta, prova a estrarle ricorsivamente
            vars.extend(self.var.extract_variables());
        }
        vars
    }
}

#[derive(Debug)]
pub struct MinusMinus {
    pub var: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
}
impl ArithmeticExpression for MinusMinus {
    type Q = AbstractInterval;
    fn to_ast(&self, abs_state: &mut AbstractState<Self::Q>, var_leaves : &mut HashMap<String, AbstractInterval>) -> Node {
        Node::Internal(
            Op::Sub,
            self.abs_evaluate(abs_state),
            Box::new(self.var.to_ast(abs_state, var_leaves) ),
            Box::new((Numeral(1)).to_ast(abs_state, var_leaves)),
        )
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn clone_box(&self) -> Box<dyn ArithmeticExpression<Q = Self::Q>> {
        Box::new(MinusMinus {
            var: self.var.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> i64 {
        //Variable evaluation -> i64
        let mut value = self.var.evaluate(state);
        value -= 1;
        state.insert(self.var.clone_box().to_string(), value);
        value
    }
    fn as_variable(&self) -> Option<&Variable> {
        self.var.as_variable()
    }
    fn to_string(&self) -> String {
        format!("{}--", self.var.to_string())
    }
    fn abs_evaluate(&self, abs_state: &mut AbstractState<Self::Q>) -> AbstractInterval {
        // println!("minus minus evaluation");
        let m = *M.lock().unwrap();
        let n = *N.lock().unwrap();
        let value = self.var.abs_evaluate(abs_state);
        match value {
            AbstractInterval::Bottom => AbstractInterval::Bottom,
            AbstractInterval::Top => AbstractInterval::Top,
            AbstractInterval::Bounded { lower, upper } => {
                if lower > m {
                    if upper < n {
                        // println!("normal case in minus minus {}", value);
                        let newlower = lower - 1;
                        // println!("newlower in minus minus {}", newlower);
                        let new_interval = AbstractInterval::new(newlower, upper);
                        // print!("new interval in minus minus {}", new_interval);
                        //let new_value = AbstractDomain::new(new_interval);
                        // abs_state
                        //     .variables
                        //     .insert(self.var.to_string(), new_value);
                        abs_state.update_interval(self.var.to_string().as_str(), new_interval);
                        new_interval
                    } else {
                        let new_int = AbstractInterval::Bounded { lower, upper: n };
                        // abs_state.variables.insert(self.var.to_string(), abstract_domain::AbstractDomain::new(new_int));
                        abs_state.update_interval(self.var.to_string().as_str(), new_int);
                        // abs_state.bottom();
                        new_int
                    }
                } else {
                    let new_int = AbstractInterval::Bounded { lower: m, upper };
                    // abs_state.variables.insert(self.var.to_string(), abstract_domain::AbstractDomain::new(new_int));
                    abs_state.update_interval(self.var.to_string().as_str(), new_int);
                    // abs_state.bottom();
                    new_int
                }
            }
        }
    }
    fn extract_variables(&self) -> Vec<&Variable> {
        let mut vars = Vec::new();
        // Se il lato sinistro è una variabile o contiene variabili
        if let Some(v) = self.var.as_variable() {
            vars.push(v);
        } else {
            // Se non è una variabile diretta, prova a estrarle ricorsivamente
            vars.extend(self.var.extract_variables());
        }
        vars
    }
}
