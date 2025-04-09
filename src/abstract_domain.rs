use core::fmt;
use std::{
    collections::HashSet, fmt::Display, ops::{Add, Div, Mul, Neg, Sub}
};

use num_traits::Zero;

use crate::abstract_interval::AbstractInterval;

pub trait AbstractDomainOps:
    Sized + PartialEq + PartialOrd + Add + Mul + Sub + Div + Neg + Ord
{
    fn lub(&self, other: &Self) -> Self;
    fn widening(&self, other: &Self) -> Self;
    fn narrowing(&self, other: &Self) -> Self;
    fn glb(&self, other: &Self) -> Self;
    fn top() -> Self;
    fn _is_top(&self) -> bool;
    fn is_bottom(&self) -> bool;
    fn _gamma(abstract_val: &Self) -> HashSet<ConcreteValue>;
    fn _alpha(r : HashSet<ConcreteValue>) -> HashSet<AbstractValue>;
}
#[derive(Debug, Clone, Copy)]
pub struct AbstractDomain<Q> {
    pub value: Q,
}
#[derive(Hash, Eq, PartialEq, Debug)]
pub struct AbstractValue{
    pub value: AbstractInterval
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct ConcreteValue{
    pub value: i64
}

impl ConcreteValue  {
    pub fn _alpha(c: ConcreteValue) -> AbstractValue{
        AbstractValue { value: AbstractInterval::Bounded { lower: c.value, upper: c.value } }
       }
}
impl<Q: AbstractDomainOps> AbstractDomain<Q> {
    // Costruttore
    pub fn new(value: Q) -> Self {
        AbstractDomain { value }
    }
    pub fn get_value(&self) -> &Q {
        &self.value
    }

    // Least upper bound (lub)
    pub fn lub(&self, other: &Self) -> Self {
        AbstractDomain {
            value: self.value.lub(&other.value),
        }
    }
    pub fn glb(&self, other: &Self) -> Self {
        AbstractDomain {
            value: self.value.glb(&other.value),
        }
    }

    // Widening
    pub fn widening(&self, other: &Self) -> Self {
        AbstractDomain {
            value: self.value.widening(&other.value),
        }
    }

    // Narrowing
    pub fn narrowing(&self, other: &Self) -> Self {
        AbstractDomain {
            value: self.value.narrowing(&other.value),
        }
    }
    pub fn is_bottom(&self) -> bool {
        self.get_value().is_bottom()
    }
    pub fn _is_top(&self) -> bool {
        self.get_value()._is_top()
    }
    //  Concretization function
    pub fn _gamma(&self) -> HashSet<ConcreteValue>{
      Q::_gamma(&self.value)
    }
    pub fn _alpha(r : HashSet<ConcreteValue>) -> HashSet<AbstractValue>{
        Q::_alpha(r)
    }
    
}

impl<Q: Copy + Ord + From<i64> + PartialEq + Zero + Display> fmt::Display for AbstractDomain<Q> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "  {} ", self.value.to_string())
    }
}
