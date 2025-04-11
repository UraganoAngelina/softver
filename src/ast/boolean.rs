use crate::abstract_domain::AbstractDomainOps;
use crate::ast::arithmetic::ArithmeticExpression;
use crate::ast::State;
use crate::{abstract_interval::AbstractInterval, abstract_state::AbstractState};
use std::fmt::Debug;

use super::arithmetic::{Add, Minus, Numeral};

pub trait BooleanExpression: Debug {
    type Q: AbstractDomainOps + PartialEq + Clone + Debug;
    fn clone_box(&self) -> Box<dyn BooleanExpression<Q = Self::Q>>;
    fn evaluate(&self, state: &mut State) -> bool;
    fn abs_evaluate(
        &self,
        state: &mut AbstractState<Self::Q>,
        flag: bool,
    ) -> AbstractState<Self::Q>;
    fn to_string(&self) -> String;
    // fn build_ast(&self , state: &mut AbstractState<Self::Q>) ->
}

#[derive(Debug)]
pub struct Boolean(pub bool);

impl BooleanExpression for Boolean {
    type Q = AbstractInterval;
    fn clone_box(&self) -> Box<dyn BooleanExpression<Q = Self::Q>> {
        Box::new(Boolean(self.0)) // Crea un nuovo Box con una copia di Numeral
    }
    fn evaluate(&self, _state: &mut State) -> bool {
        self.0
    }
    fn abs_evaluate(&self, state: &mut AbstractState<Self::Q>, flag: bool) -> AbstractState<Self::Q>
    where
        Self::Q: AbstractDomainOps + Clone + PartialEq,
    {
        if !flag {
            if self.0 {
                state.clone()
            } else {
                AbstractState::bottom(state)
            }
        } else {
            if self.0 {
                AbstractState::bottom(state)
            } else {
                state.clone()
            }
        }
    }
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug)]
pub struct Equal {
    pub left: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
    pub right: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
}

impl BooleanExpression for Equal {
    type Q = AbstractInterval;
    fn clone_box(&self) -> Box<dyn BooleanExpression<Q = Self::Q>> {
        Box::new(Equal {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> bool {
        self.left.evaluate(state) == self.right.evaluate(state)
    }
    fn abs_evaluate(
        &self,
        state: &mut AbstractState<Self::Q>,
        flag: bool,
    ) -> AbstractState<Self::Q> {
        if self.left.abs_evaluate(state).is_bottom() || self.right.abs_evaluate(state).is_bottom() {
            AbstractState::bottom(state)
        } else {
            //(a - b <= 0) and (b - a <=0)
            let left = Box::new(LessEqual {
                left: Box::new(Minus {
                    left: self.left.clone_box(),
                    right: self.right.clone_box(),
                }),
                right: Box::new(Numeral(0)),
            });
            let right = Box::new(LessEqual {
                left: Box::new(Minus {
                    left: self.right.clone_box(),
                    right: self.left.clone_box(),
                }),
                right: Box::new(Numeral(0)),
            });
            let canonical = Box::new(And { left, right });
            canonical.abs_evaluate(state, flag)
        }
    }
    fn to_string(&self) -> String {
        format!("{} = {}", self.left.to_string(), self.right.to_string())
    }
}
#[derive(Debug)]
pub struct NotEqual {
    pub left: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
    pub right: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
}

impl BooleanExpression for NotEqual {
    type Q = AbstractInterval;
    fn clone_box(&self) -> Box<dyn BooleanExpression<Q = Self::Q>> {
        Box::new(NotEqual {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }

    fn evaluate(&self, state: &mut State) -> bool {
        self.left.evaluate(state) != self.right.evaluate(state)
    }

    fn abs_evaluate(
        &self,
        state: &mut AbstractState<Self::Q>,
        flag: bool,
    ) -> AbstractState<Self::Q> {
        // (b - a + 1 <=0) or (a - b + 1 <=0)
        let left = Box::new(LessEqual {
            left: Box::new(Add {
                left: Box::new(Minus {
                    left: self.right.clone_box(),
                    right: self.left.clone_box(),
                }),
                right: Box::new(Numeral(1)),
            }),
            right: Box::new(Numeral(0)),
        });
        let right = Box::new(LessEqual {
            left: Box::new(Add {
                left: Box::new(Minus {
                    left: self.left.clone_box(),
                    right: self.right.clone_box(),
                }),
                right: Box::new(Numeral(1)),
            }),
            right: Box::new(Numeral(0)),
        });
        let canonical = Box::new(Or { left, right });
        canonical.abs_evaluate(state, flag)
    }
    fn to_string(&self) -> String {
        format!("{} != {}", self.left.to_string(), self.right.to_string())
    }
}

#[derive(Debug)]
pub struct GreatEqual {
    pub left: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
    pub right: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
}

impl BooleanExpression for GreatEqual {
    type Q = AbstractInterval;
    fn clone_box(&self) -> Box<dyn BooleanExpression<Q = Self::Q>> {
        Box::new(GreatEqual {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> bool {
        self.left.evaluate(state) >= self.right.evaluate(state)
    }
    fn abs_evaluate(
        &self,
        state: &mut AbstractState<Self::Q>,
        flag: bool,
    ) -> AbstractState<Self::Q> {
        if self.left.abs_evaluate(state).is_bottom() || self.right.abs_evaluate(state).is_bottom() {
            AbstractState::bottom(state);
        }

        // b - a  <= 0
        let lhs = Box::new(Minus {
            left: self.right.clone_box(),
            right: self.left.clone_box(),
        });

        let rhs = Box::new(Numeral(0));
        let canonical = Box::new(LessEqual {
            left: lhs,
            right: rhs,
        });
        canonical.abs_evaluate(state, flag)
    }
    fn to_string(&self) -> String {
        format!("{} >= {}", self.left.to_string(), self.right.to_string())
    }
}
#[derive(Debug)]
pub struct Great {
    pub left: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
    pub right: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
}
impl BooleanExpression for Great {
    type Q = AbstractInterval;
    fn clone_box(&self) -> Box<dyn BooleanExpression<Q = Self::Q>> {
        Box::new(Great {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> bool {
        self.left.evaluate(state) > self.right.evaluate(state)
    }
    fn abs_evaluate(
        &self,
        state: &mut AbstractState<Self::Q>,
        _flag: bool,
    ) -> AbstractState<Self::Q> {
        if self.left.abs_evaluate(state).is_bottom() || self.right.abs_evaluate(state).is_bottom() {
            AbstractState::bottom(state);
        }
        
        // map( fn : AST -> BTree )
        // if.map(fn){return fn(new If(fn(guard), fn(then), fn(else)));}
        
        // Bexpr =          (e)            <= 0
        //          ArithmeticExpression
        // evaluate
        // const evaluate = (aExpr: ArithmeticExpression, aState: AbstractProgramState<T>): BinaryTree<T> => {

        //     if (aExpr instanceof ArithmeticBinaryOperator) {
        //         return new BinaryNode(
        //             this.E(aExpr, aState).value,
        //             evaluate(aExpr.leftOperand, this.E(aExpr, aState).state),
        //             evaluate(aExpr.rightOperand, this.E(aExpr, aState).state),
        //             aExpr.operator.value
        //         )
        //     }
        //     if (aExpr instanceof ArithmeticUnaryOperator) {
        //         return new UnaryNode(
        //             this.E(aExpr, aState).value,
        //             evaluate(aExpr.operand, this.E(aExpr, aState).state),
        //         )
        //     }
        //     if (aExpr instanceof IncrementOperator || aExpr instanceof DecrementOperator) {
        //         return new LeafNode(
        //             this.E(aExpr, aState).value,
        //         )
        //     }
        //     if (aExpr instanceof Variable) {
        //         return new VariableNode(this.E(aExpr, aState).value, aExpr.name);
        //     } else {
        //         return new LeafNode(this.E(aExpr, aState).value)
        //     }
        // }

        // Intersect : BackworkOperator.leq(Btree)

        // Propagate:
        // const propagate = (node: BinaryTree<T>): BinaryTree<T> => {
        //     if (node instanceof VariableNode) {
        //         return node;
        //     } else if (node instanceof LeafNode) {
        //         return node;
        //     } else if (node instanceof UnaryNode) {
        //         let ret = node.clone(node.data)
        //         ret.child = propagate(node.child.clone(this.BackwardOperators.negate(node.child.data, node.data)));
        //         return ret;
        //     } else {
        //         let aux;
        //         let bNode = node as BinaryNode<T>;
        //         switch ((bNode as BinaryNode<T>).operator) {
        //             case "+":
        //                 aux = this.BackwardOperators.add(bNode.left.data, bNode.right.data, bNode.data);
        //                 break;
        //             case "-":
        //                 aux = this.BackwardOperators.subtract(bNode.left.data, bNode.right.data, bNode.data);
        //                 break;
        //             case "*":
        //                 aux = this.BackwardOperators.multiply(bNode.left.data, bNode.right.data, bNode.data);
        //                 break;
        //             case "/":
        //                 aux = this.BackwardOperators.divide(bNode.left.data, bNode.right.data, bNode.data);
        //                 break;
        //         };
        //         let ret = bNode.clone(bNode.data);
        //         ret.left = propagate(bNode.left.clone(aux?.x));
        //         ret.right = propagate(bNode.right.clone(aux?.y));
        //         return ret;
        //     }
        // }
        
        // BackwardOperators = {
        //     leqZero: (x: Interval): Interval => {
        //         return this.SetOperators.intersection(x, this._IntervalFactory.getLessThanOrEqual(0));
        //     },
        //     negate: (x: Interval, y: Interval): Interval => {
        //         return this.SetOperators.intersection(x, this.Operators.negate(y));
        //     },
        //     add: (x: Interval, y: Interval, r: Interval): { x: Interval; y: Interval; } => {
        //         return {
        //             x: this.SetOperators.intersection(x, this.Operators.subtract(r, y)),
        //             y: this.SetOperators.intersection(y, this.Operators.subtract(r, x)),
        //         }
        //     },
        //     subtract: (x: Interval, y: Interval, r: Interval): { x: Interval; y: Interval; } => {
        //         return {
        //             x: this.SetOperators.intersection(x, this.Operators.add(r, y)),
        //             y: this.SetOperators.intersection(y, this.Operators.subtract(x, r)),
        //         }
        //     },
        //     multiply: (x: Interval, y: Interval, r: Interval): { x: Interval; y: Interval; } => {
        //         return {
        //             x: this.SetOperators.intersection(x, this.Operators.divide(r, y)),
        //             y: this.SetOperators.intersection(y, this.Operators.divide(r, x)),
        //         }
        //     },
        //     divide: (x: Interval, y: Interval, r: Interval): { x: Interval; y: Interval; } => {
        //         let s = this.Operators.add(r, this._IntervalFactory.new(-1, 1));
        //         return {
        //             x: this.SetOperators.intersection(x, this.Operators.multiply(s, y)),
        //             y: this.SetOperators.intersection(y, this.SetOperators.union(this.Operators.divide(x, s), this._IntervalFactory.new(0, 0))),
        //         }
        //     }
        // };

        // public Operators = {
        //     negate: (x: Interval): Interval => {
        //         return this.new(-x.upper, -x.lower)
        //     },
        //     add: (x: Interval, y: Interval): Interval => {
        //         if (x.isBottom() || y.isBottom()) return this.Bottom;
        //         const l = x.lower + y.lower;
        //         const u = x.upper + y.upper;
        //         return this.new(l, u);
        //     },
        //     subtract: (x: Interval, y: Interval): Interval => {
        //         if (x.isBottom() || y.isBottom()) return this.Bottom;
        //         const l = x.lower - y.upper;
        //         const u = x.upper - y.lower;
        //         return this.new(l, u);
        //     },
        //     multiply: (x: Interval, y: Interval): Interval => {
        //         if (x.isBottom() || y.isBottom()) return this.Bottom;
        //         const products: Array<number> = [
        //             x.lower * y.lower, x.lower * y.upper,
        //             x.upper * y.lower, x.upper * y.upper
        //         ];
        //         return this.new(Math.min(...products), Math.max(...products));
        //     },
        //     divide: (x: Interval, y: Interval): Interval => {
        //         if (x.isBottom() || y.isBottom()) return this.Bottom;
        //         if (1 <= y.lower) {
        //             const l = Math.min(x.lower / y.lower, x.lower / y.upper);
        //             const u = Math.max(x.upper / y.lower, x.upper / y.upper)
        //             return this.new(l, u);
        //         } else if (y.upper <= -1) {
        //             const l = Math.min(x.upper / y.lower, x.upper / y.upper);
        //             const u = Math.max(x.lower / y.lower, x.lower / y.upper);
        //             return this.new(l, u);
        //         } return this.union(
        //             this.Operators.divide(x, this.intersect(y, this.getMoreThan(0))),
        //             this.Operators.divide(x, this.intersect(y, this.getLessThan(0)))
        //         )
        //     }
        // };


        // b - a + 1 <= 0
        let lhs = Box::new(Add {
            left: Box::new(Minus {
                left: self.right.clone_box(),
                right: self.left.clone_box(),
            }),
            right: Box::new(Numeral(1)),
        });
        let rhs = Box::new(Numeral(0));
        let canonical = Box::new(LessEqual {
            left: lhs,
            right: rhs,
        });

        canonical.abs_evaluate(state, false)
    }
    fn to_string(&self) -> String {
        format!("{} > {}", self.left.to_string(), self.right.to_string())
    }
}

#[derive(Debug)]
pub struct LessEqual {
    pub left: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
    pub right: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
}

impl BooleanExpression for LessEqual {
    type Q = AbstractInterval;
    fn clone_box(&self) -> Box<dyn BooleanExpression<Q = Self::Q>> {
        Box::new(LessEqual {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> bool {
        self.left.evaluate(state) <= self.right.evaluate(state)
    }
    fn abs_evaluate(
        &self,
        state: &mut AbstractState<Self::Q>,
        flag: bool,
    ) -> AbstractState<Self::Q> {
        if !flag {
            println!(
                "less equal {} <= {} normal eval in state {}",
                self.left.to_string(),
                self.right.to_string(),
                state
            );
            if self.left.abs_evaluate(state).is_bottom()
                || self.right.abs_evaluate(state).is_bottom()
            {
                AbstractState::bottom(state);
            }

            let left_eval = self.left.abs_evaluate(state);
            let right_eval = self.right.abs_evaluate(state);
            println!(
                "evaluation lhs: {} rhs:{}",
                left_eval.to_string(),
                right_eval.to_string()
            );
            // let left_var = self.left.extract_variables();
            //    for element in left_var {
            //     state.update_interval(&element.value, left_eval);
            //    }

            match (left_eval, right_eval) {
                // Caso base: uno dei due è Bottom
                (AbstractInterval::Bottom, _) | (_, AbstractInterval::Bottom) => {
                    println!("bottom case of leq");
                    AbstractState::bottom(state)
                }

                // Caso concreto: Entrambi gli intervalli sono bounded
                (
                    AbstractInterval::Bounded {
                        lower: l1,
                        upper: u1,
                    },
                    AbstractInterval::Bounded {
                        lower: l2,
                        upper: u2,
                    },
                ) => {
                    // caso x <= y (y var)
                    if let (Some(left_var), Some(_right_var)) =
                        (self.left.as_variable(), self.right.as_variable())
                    {
                        if l1 <= u2 {
                            // Aggiorniamo l'intervallo della variabile sinistra
                            state.update_interval(
                                &left_var.value,
                                AbstractInterval::Bounded {
                                    lower: l1,
                                    upper: std::cmp::min(u1, u2),
                                },
                            );
                            state.update_interval(
                                &_right_var.value,
                                AbstractInterval::Bounded {
                                    lower: std::cmp::min(l1, l2),
                                    upper: u1,
                                },
                            );
                            println!(" leq result {}", state.clone());
                            state.clone()
                        } else {
                            // Non soddisfatto: Bottom

                            state.update_interval(&left_var.value, AbstractInterval::Bottom);
                            println!(" leq result {}", state.clone());
                            AbstractState::bottom(state)
                        }
                    } else if let (Some(left_var), Some(_right_num)) = (
                        self.left.as_variable(),
                        self.right.as_any().downcast_ref::<Numeral>(),
                    ) {
                        if l1 <= u2 {
                            // Aggiorniamo l'intervallo della variabile sinistra
                            state.update_interval(
                                &left_var.value,
                                AbstractInterval::Bounded {
                                    lower: l1,
                                    upper: std::cmp::min(u1, u2),
                                },
                            );
                            println!(" leq result {}", state.clone());
                            state.clone()
                        } else {
                            // Non soddisfatto: Bottom
                            state.update_interval(&left_var.value, AbstractInterval::Bottom);
                            println!(" leq result {}", state.clone());
                            AbstractState::bottom(state)
                        }
                    } else {
                        // Se il lato sinistro non è una variabile, ritorno lo stato iniziale
                        state.clone()
                    }
                }
                (AbstractInterval::Top, _) | (_, AbstractInterval::Top) => {
                    println!(" top case res{}", state.clone());
                    state.clone()
                }
            }
        } else {
            println!("filtering with !guard in leq in state {}", state);
            if self.left.abs_evaluate(state).is_bottom()
                || self.right.abs_evaluate(state).is_bottom()
            {
                println!(" bottom case in leq negated");
                AbstractState::bottom(state);
            }
            println!(
                "lhs  rhs {} > {} in final filtering",
                self.left.to_string(),
                self.right.to_string()
            );
            if state.is_bottom() {
                return AbstractState::bottom(&state);
            }
            let gr = Great {
                left: self.left.clone_box(),
                right: self.right.clone_box(),
            };
            let my_flag = !flag;
            gr.abs_evaluate(state, my_flag)
        }
    }
    fn to_string(&self) -> String {
        format!("{} <= {}", self.left.to_string(), self.right.to_string())
    }
}

#[derive(Debug)]
pub struct Less {
    pub left: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
    pub right: Box<dyn ArithmeticExpression<Q = AbstractInterval>>,
}

impl BooleanExpression for Less {
    type Q = AbstractInterval;
    fn clone_box(&self) -> Box<dyn BooleanExpression<Q = Self::Q>> {
        Box::new(Less {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> bool {
        self.left.evaluate(state) < self.right.evaluate(state)
    }
    fn abs_evaluate(
        &self,
        state: &mut AbstractState<Self::Q>,
        flag: bool,
    ) -> AbstractState<Self::Q> {
        if self.left.abs_evaluate(state).is_bottom() || self.right.abs_evaluate(state).is_bottom() {
            AbstractState::bottom(state);
        }
        // println!("Interpreting {:?}", self);
        // println!("In the abstract state {}", state);
        let zero_int = AbstractInterval::Bounded { lower: 0, upper: 0 };
        if self.right.abs_evaluate(state) == zero_int {
            let lhs = Box::new(Add {
                left: self.left.clone_box(),
                right: Box::new(Numeral(1)),
            });
            let rhs = Box::new(Numeral(0));
            let canonical = Box::new(LessEqual {
                left: lhs,
                right: rhs,
            });
            canonical.abs_evaluate(state, flag)
        } else {
            let lhs = Box::new(Add {
                left: Box::new(Minus {
                    left: self.left.clone_box(),
                    right: self.right.clone_box(),
                }),
                right: Box::new(Numeral(1)),
            });
            let rhs = Box::new(Numeral(0));
            let canonical = Box::new(LessEqual {
                left: lhs,
                right: rhs,
            });
            canonical.abs_evaluate(state, flag)
        }
    }
    fn to_string(&self) -> String {
        format!("{} < {}", self.left.to_string(), self.right.to_string())
    }
}

#[derive(Debug)]
pub struct And {
    pub left: Box<dyn BooleanExpression<Q = AbstractInterval>>,
    pub right: Box<dyn BooleanExpression<Q = AbstractInterval>>,
}

impl BooleanExpression for And {
    type Q = AbstractInterval;
    fn clone_box(&self) -> Box<dyn BooleanExpression<Q = Self::Q>> {
        Box::new(And {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> bool {
        self.left.evaluate(state) && self.right.evaluate(state)
    }
    fn abs_evaluate(
        &self,
        state: &mut AbstractState<Self::Q>,
        flag: bool,
    ) -> AbstractState<Self::Q> {
        if !flag {
            let mut x = state.clone();
            let left_eval = self.left.abs_evaluate(&mut x, false);
            let right_eval = self.right.abs_evaluate(&mut x, false);
            if left_eval.is_bottom() || right_eval.is_bottom() {
                return AbstractState::bottom(state);
            }
            let current = left_eval.state_glb(&right_eval);
            state.variables.extend(current.variables.clone());
            current
        } else {
            let mut x = state.clone();

            let left_eval = self.left.abs_evaluate(&mut x, false);
            let right_eval = self.right.abs_evaluate(&mut x, false);
            if left_eval.is_bottom() || right_eval.is_bottom() {
                return AbstractState::bottom(state);
            }
            let current = left_eval.state_lub(&right_eval);
            state.variables.extend(current.variables.clone());
            current
        }
    }
    fn to_string(&self) -> String {
        format!("{} && {}", self.left.to_string(), self.right.to_string())
    }
}

#[derive(Debug)]
pub struct Or {
    pub left: Box<dyn BooleanExpression<Q = AbstractInterval>>,
    pub right: Box<dyn BooleanExpression<Q = AbstractInterval>>,
}

impl BooleanExpression for Or {
    type Q = AbstractInterval;
    fn clone_box(&self) -> Box<dyn BooleanExpression<Q = Self::Q>> {
        Box::new(Or {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> bool {
        self.left.evaluate(state) || self.right.evaluate(state)
    }
    fn abs_evaluate(
        &self,
        state: &mut AbstractState<Self::Q>,
        flag: bool,
    ) -> AbstractState<Self::Q> {
        if !flag {
            let left_eval = self.left.abs_evaluate(state, false);
            let right_eval = self.right.abs_evaluate(state, false);
            if left_eval.is_bottom() || right_eval.is_bottom() {
                return AbstractState::bottom(state);
            }
            let ret = left_eval.state_lub(&right_eval);

            state.variables.extend(ret.variables.clone());
            ret
        } else {
            let left_eval = self.left.abs_evaluate(state, false);
            let right_eval = self.right.abs_evaluate(state, false);
            if left_eval.is_bottom() || right_eval.is_bottom() {
                return AbstractState::bottom(state);
            }
            let ret = left_eval.state_glb(&right_eval);
            state.variables.extend(ret.variables.clone());
            ret
        }
    }
    fn to_string(&self) -> String {
        format!("{} || {}", self.left.to_string(), self.right.to_string())
    }
}

#[derive(Debug)]
pub struct Not {
    pub expression: Box<dyn BooleanExpression<Q = AbstractInterval>>,
}

impl BooleanExpression for Not {
    type Q = AbstractInterval;
    fn clone_box(&self) -> Box<dyn BooleanExpression<Q = Self::Q>> {
        Box::new(Not {
            expression: self.expression.clone_box(),
        })
    }
    fn evaluate(&self, state: &mut State) -> bool {
        !(self.expression.evaluate(state))
    }
    fn abs_evaluate(
        &self,
        state: &mut AbstractState<Self::Q>,
        flag: bool,
    ) -> AbstractState<Self::Q> {
        if self.expression.abs_evaluate(state, flag).is_bottom() {
            AbstractState::bottom(state);
        }
        let myflag = !flag;
        let expr_eval = self.expression.abs_evaluate(state, myflag);
        expr_eval
    }

    fn to_string(&self) -> String {
        format!("! {}", self.expression.to_string())
    }
}
