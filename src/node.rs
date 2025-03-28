use core::panic;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{abstract_domain::{AbstractDomain, AbstractDomainOps}, abstract_interval::AbstractInterval, abstract_state::AbstractState, ast::arithmetic::{ArithmeticExpression, Numeral}};
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

pub enum Node<Q: AbstractDomainOps> {
    Internal {
        value: RefCell<AbstractDomain<Q>>,
        operator: Operator,
        left: Rc<Node<Q>>,
        right: Rc<Node<Q>>,
    },
    VarLeaf {
        value: RefCell<AbstractDomain<Q>>,
    },
    ConstantLeaf {
        value: AbstractDomain<Q>,
    },
}

impl<Q: AbstractDomainOps + Copy + From<i64>> Node<Q> {
    pub fn build<'a,  E: ArithmeticExpression<Q = AbstractInterval>>(
        exp: &E,
        state: &AbstractState<Q>,
        var_leafs: &mut HashMap<&'a str, Rc<Self>>,
    ) -> Rc<Self> {
        // match exp {
        //     ArithmeticExp::Integer(c) => Rc::new(Node::ConstantLeaf {
        //         value: AbstractDomain::new(Q::from(*c)), // Adattato per incapsulare il valore in AbstractDomain
        //     }),
        //     ArithmeticExp::Variable(var) => {
        //         let node = Rc::new(Node::VarLeaf {
        //             value: RefCell::new(state.lookup(var)),
        //         });
        //         var_leafs.insert(var, Rc::clone(&node));
        //         node
        //     }
        //     ArithmeticExp::BinaryOperation { lhs, operator, rhs } => Rc::new(Node::Internal {
        //         value: RefCell::new(AbstractDomain::new(Q::top())), // Adattato per usare il valore top
        //         operator: *operator,
        //         left: Self::build(lhs, state, var_leafs),
        //         right: Self::build(rhs, state, var_leafs),
        //     }),
        // }
        if let Some(int) = exp.as_any().downcast_ref::<Numeral>() {
            return Rc::new(Node::ConstantLeaf {
                value: AbstractDomain::new(Q::from(int.0)), // Adatta in base alla tua struttura
            });
        }
        panic!("dio bestia")
        
    }
}
