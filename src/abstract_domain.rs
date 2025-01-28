use core::fmt;
use std::fmt::Display;

use num_traits::Zero;

use crate::abstract_interval::AbstractDomainOps;
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

    // Relazione di ordine parziale
    fn partial_order(&self, other: &Self) -> bool {
        self.value.partial_order(&other.value)
    }

    // Least upper bound (lub)
    pub fn lub(&self, other: &Self) -> Self {
        AbstractDomain {
            value: self.value.lub(&other.value),
        }
    }

    // Greatest lower bound (glb)
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
}

impl<M: Copy + Ord + From<i64> + PartialEq + Zero + Display> fmt::Display for AbstractDomain<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, " {{ {} }}", self.value.to_string())

        }
    }
