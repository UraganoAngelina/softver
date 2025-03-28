use core::fmt;
use std::fmt::Display;

use num_traits::Zero;

pub trait AbstractDomainOps {
    fn lub(&self, other: &Self) -> Self;
    fn widening(&self, other: &Self) -> Self;
    fn narrowing(&self, other: &Self) -> Self;
    fn glb(&self, other: &Self) -> Self;
    fn top() -> Self;
    fn is_top(&self) -> bool;
    fn is_bottom(&self) -> bool;
}
#[derive(Debug, Clone, Copy)]
pub struct AbstractDomain<Q> {
    pub value: Q,
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
    pub fn is_bottom(& self) -> bool {
        self.get_value().is_bottom()
    }
    pub fn is_top(&self) -> bool {
        self.get_value().is_top()
    }
}

impl<Q: Copy + Ord + From<i64> + PartialEq + Zero + Display> fmt::Display for AbstractDomain<Q> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, " {{ {} }}", self.value.to_string())
    }
}
