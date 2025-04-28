pub mod arithmetic;
pub mod boolean;
pub mod statement;

use std::collections::HashMap;

use crate::abstract_interval::AbstractInterval;
pub type State = HashMap<String, i64>;

#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Uminus,
}

// #[derive(Debug, Clone)]
// pub enum RelOp {
//     Less,
//     LessEqual,
//     Greater,
//     GreaterEqual,
//     Equal,
//     NotEqual,
// }
#[derive(Debug, Clone)]
pub enum Node {
    Internal(Op, AbstractInterval, Box<Node>, Box<Node>),
    UInternal(Op, AbstractInterval, Box<Node>),
    ConstantLeaf(AbstractInterval),
    VarLeaf(String, AbstractInterval),
}
impl Node {
    pub fn get_value(&self) -> AbstractInterval {
        match self {
            Node::ConstantLeaf(value) => value.clone(),
            Node::Internal(_, value, _, _) | Node::VarLeaf(_, value) => value.clone(),
            Node::UInternal(_, value, _) => value.clone(),
        }
    }

    fn inner_pretty_print(&self, indent: String, last: bool) {
        let node_type = match self {
            Node::Internal(operator, _value, _left, _right) => match operator {
                Op::Add => "+".to_string(),
                Op::Sub => "-".to_string(),
                Op::Mul => "*".to_string(),
                Op::Div => "/".to_string(),
                Op::Uminus => "u-".to_string(),
            },
            Node::ConstantLeaf(value) => value.to_string(),
            Node::VarLeaf(name, value) => name.to_string() + &value.to_string(),
            Node::UInternal(op, _abstract_interval, _node) => match op {
                Op::Add => "+".to_string(),
                Op::Sub => "-".to_string(),
                Op::Mul => "*".to_string(),
                Op::Div => "/".to_string(),
                Op::Uminus => "u-".to_string(),
            },
        };

        println!(
            "{indent}{node_type} {}",
            <AbstractInterval as Into<String>>::into(self.get_value()),
        );

        let mut new_indent = format!("{indent}|  ");
        if last {
            new_indent = format!("{indent}   ");
        }

        match self {
            Node::Internal(_, _, left, right) => {
                left.inner_pretty_print(new_indent.clone(), false);
                right.inner_pretty_print(new_indent, true);
            }
            _ => (),
        }
    }

    pub fn pretty_print(&self) {
        self.inner_pretty_print("".to_string(), matches!(self, Node::Internal(_, _, _, _)));
    }

    pub fn backward_analysis(&self, refinement: AbstractInterval) -> bool {
        match self {
            Node::Internal(op, mut abstract_interval, left, right) => {
                abstract_interval = refinement;

                let refinements = AbstractInterval::backward_arithmetic_operator(
                    left.get_value(),
                    right.get_value(),
                    abstract_interval,
                    op.clone(),
                );
                left.backward_analysis(refinements[0]) && right.backward_analysis(refinements[1])
            }
            Node::UInternal(op, mut abstract_interval, node) => {
                abstract_interval = refinement;

                let refinements = AbstractInterval::backward_unary_arithmetic_operator(
                    op.clone(),
                    node.get_value(),
                    abstract_interval,
                );
                node.backward_analysis(refinements[0])
            }
            Node::ConstantLeaf(abstract_interval) => {
                refinement.intersect(abstract_interval) != AbstractInterval::Bottom
            }
            Node::VarLeaf(_, mut abstract_interval) => {
                let value = refinement.intersect(&abstract_interval);
                if value != AbstractInterval::Bottom {
                    abstract_interval = refinement;
                }
                value != AbstractInterval::Bottom
            }
        }
    }
}

// #[derive(Debug, Clone)]
// pub struct BooleanAST {
//     pub op: RelOp,
//     pub value: AbstractInterval,
//     pub child: Box<Node>,

// }
// impl BooleanAST{
//     pub fn pretty_print(&self)
//     {
//         match self.op {
//             RelOp::LessEqual => "z=".to_string(),
//             _ =>"other".to_string(),
//         };
//         self.child.pretty_print();
//     }
// }
