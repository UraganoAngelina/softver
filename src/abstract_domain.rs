use core::fmt;
use std::fmt::Display;

use num_traits::Zero;


pub trait AbstractDomainOps {
    fn lub(&self, other: &Self) -> Self;
    fn widening(&self, other: &Self) -> Self;
    fn narrowing(&self, other: &Self) -> Self;
}
#[derive(Debug, Clone, Copy)]
pub struct AbstractDomain<M> {
    pub value: M,

}

impl<M: AbstractDomainOps> AbstractDomain<M> {
    // Costruttore
    pub fn new(value: M) -> Self {
        AbstractDomain { value }
    }
    pub fn get_value(&self) -> &M {
        &self.value
    }

    // Least upper bound (lub)
    pub fn lub(&self, other: &Self) -> Self {
        AbstractDomain {
            value: self.value.lub(&other.value),
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
}

impl<M: Copy + Ord + From<i64> + PartialEq + Zero + Display> fmt::Display for AbstractDomain<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, " {{ {} }}", self.value.to_string())

        }
    }
