use num_traits::Zero;
use std::cmp::{Ordering, PartialOrd};
use std::collections::HashSet;
use std::fmt::{self, Debug};
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::abstract_domain::{AbstractDomainOps, AbstractValue, ConcreteValue};
use crate::N;
use crate::{find_max, find_min, CONSTANTS_VECTOR, M};

#[derive(Debug, Clone, Copy, Eq, Hash)]
pub enum AbstractInterval {
    Bottom,                             // Stuck configuration
    Top,                                // Lack of information
    Bounded { lower: i64, upper: i64 }, // Regular Interval
}

impl AbstractDomainOps for AbstractInterval {
    fn lub(&self, other: &Self) -> Self {
        self.int_lub(other)
    }

    fn widening(&self, other: &Self) -> Self {
        self.int_widening(other)
    }

    fn narrowing(&self, other: &Self) -> Self {
        self.int_narrowing(other)
    }

    fn _is_top(&self) -> bool {
        self._is_top()
    }

    fn glb(&self, other: &Self) -> Self {
        self.intersect(other)
    }

    fn is_bottom(&self) -> bool {
        self.is_bottom()
    }

    fn top() -> Self {
        AbstractInterval::top()
    }

    fn _gamma(abstract_val: &Self) -> HashSet<ConcreteValue> {
        abstract_val._gamma()
    }

    fn _alpha(r: HashSet<ConcreteValue>) -> HashSet<AbstractValue> {
        if r.is_empty() {
            return HashSet::new();
        }
        r.into_iter().map(|c| AbstractInterval::alpha(c)).collect()
    }
}

impl fmt::Display for AbstractInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AbstractInterval::Bottom => write!(f, "Bottom ┴"),
            AbstractInterval::Top => write!(f, "Top ┬"),
            AbstractInterval::Bounded { lower, upper } => {
                let m = *M.lock().expect("failed to lock m mutex");
                let n = *N.lock().expect("failed to lock n mutex");
                if *lower == m && *upper != n {
                    write!(f, "[-∞, {}]", upper)
                } else if *upper == n && *lower != m {
                    write!(f, "[{}, +∞]", lower)
                } else if *upper == n && *lower == m {
                    write!(f, "[-∞, +∞]")
                } else {
                    write!(f, "[{}, {}]", lower, upper)
                }
            }
        }
    }
}

impl AbstractInterval {
    /// Create an interval only if is well defined
    pub fn new(lower: i64, upper: i64) -> Self {
        if lower > upper {
            Self::Bottom
        } else {
            Self::Bounded { lower, upper }
        }
    }
    pub fn alpha(c: ConcreteValue) -> AbstractValue {
        AbstractValue {
            value: AbstractInterval::Bounded {
                lower: c.value,
                upper: c.value,
            },
        }
    }
    pub fn _gamma(&self) -> HashSet<ConcreteValue> {
        let m_val = *M.lock().expect("Failed to lock M");
        let n_val = *N.lock().expect("Failed to lock N");
        match self {
            AbstractInterval::Bottom => HashSet::new(), // Empty Set
            AbstractInterval::Top => (m_val..=n_val)
                .map(|v| ConcreteValue { value: v }) // Whole Domain Set
                .collect(),
            AbstractInterval::Bounded { lower, upper } => (*lower..=*upper)
                .map(|v| ConcreteValue { value: v })
                .collect(),
        }
    }
    /// Interval Least Upper Bound
    pub fn int_lub(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Bottom, x) | (x, Self::Bottom) => x.clone(),
            (Self::Top, _) | (_, Self::Top) => Self::Top,
            (
                Self::Bounded {
                    lower: l1,
                    upper: u1,
                },
                Self::Bounded {
                    lower: l2,
                    upper: u2,
                },
            ) => {
                let m = *M.lock().unwrap();
                let n = *N.lock().unwrap();
                if m == n {
                    let new_lower = m;
                    let new_upper = m;
                    Self::Bounded {
                        lower: new_lower,
                        upper: new_upper,
                    }
                } else {
                    let new_lower = *l1.min(l2);
                    let new_upper = *u1.max(u2);
                    if new_lower <= new_upper {
                        Self::Bounded {
                            lower: new_lower,
                            upper: new_upper,
                        }
                    } else {
                        Self::Bottom
                    }
                }
            }
        }
    }

    // Interval widening
    pub fn int_widening(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Bottom, x) | (x, Self::Bottom) => x.clone(),
            (Self::Top, _) | (_, Self::Top) => Self::Top,
            (
                Self::Bounded {
                    lower: l1,
                    upper: u1,
                },
                Self::Bounded {
                    lower: l2,
                    upper: u2,
                },
            ) => {
                let m = *M.lock().expect("failed to lock m mutex");
                let n = *N.lock().expect("failed to lock n mutex");
                let mut vec = CONSTANTS_VECTOR
                    .lock()
                    .expect("failed to lock constant vector");
                if m == n {
                    let new_lower = m;
                    let new_upper = m;
                    Self::Bounded {
                        lower: new_lower,
                        upper: new_upper,
                    }
                } else {
                    let new_lower = if l1 <= &l2 {
                        *l1
                    } else {
                        //threshold research
                        find_max(&mut vec, l2.clone())
                    };

                    let new_upper = if u1 >= &u2 {
                        *u1
                    } else {
                        //threshold research
                        find_min(&mut vec, u2.clone())
                    };
                    Self::Bounded {
                        lower: new_lower,
                        upper: new_upper,
                    }
                }
            }
        }
    }

    // Interval narrowing
    pub fn int_narrowing(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Bottom, x) | (x, Self::Bottom) => x.clone(),
            (Self::Top, x) | (x, Self::Top) => x.clone(),
            (
                Self::Bounded {
                    lower: l1,
                    upper: u1,
                },
                Self::Bounded {
                    lower: l2,
                    upper: u2,
                },
            ) => {
                //dovrei controllare il caso in cui il mio M sia == N
                let m = *M.lock().unwrap();
                let n = *N.lock().unwrap();
                if m == n {
                    let new_lower = m;
                    let new_upper = m;
                    Self::Bounded {
                        lower: new_lower,
                        upper: new_upper,
                    }
                } else {
                    let new_lower = if m >= *l1 { *l2 } else { *l1 };
                    let new_upper = if n <= *u1 { *u2 } else { *u1 };
                    Self::Bounded {
                        lower: new_lower,
                        upper: new_upper,
                    }
                }
            }
        }
    }

    /// Intersezione di due intervalli
    pub fn intersect(&self, other: &Self) -> Self {
        // Pattern matching per gestire i casi
        match (self, other) {
            (Self::Bottom, _) | (_, Self::Bottom) => Self::Bottom,
            (Self::Top, x) | (x, Self::Top) => x.clone(),
            (
                Self::Bounded {
                    lower: l1,
                    upper: u1,
                },
                Self::Bounded {
                    lower: l2,
                    upper: u2,
                },
            ) => {
                let m_val = *M.lock().expect("Failed to lock M");
                let n_val = *N.lock().expect("Failed to lock N");

                if m_val == n_val {
                    Self::Bounded {
                        lower: m_val,
                        upper: m_val,
                    }
                } else {
                    let new_lower = *l1.max(l2);
                    let new_upper = *u1.min(u2);

                    if new_lower <= new_upper {
                        Self::Bounded {
                            lower: new_lower,
                            upper: new_upper,
                        }
                    } else {
                        Self::Bottom
                    }
                }
            }
        }
    }
    pub fn _is_top(&self) -> bool {
        let m = *M.lock().unwrap();
        let n = *N.lock().unwrap();
        match self {
            Self::Top => true,
            Self::Bounded { lower, upper } => {
                if *lower == m && *upper == n {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
    pub fn is_bottom(&self) -> bool {
        match self {
            Self::Bottom => true,
            _ => false,
        }
    }
    pub fn top() -> Self {
        AbstractInterval::Top
    }
}

impl PartialEq for AbstractInterval {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bottom, Self::Bottom) => true,
            (Self::Top, Self::Top) => true,
            (
                Self::Bounded {
                    lower: l1,
                    upper: u1,
                },
                Self::Bounded {
                    lower: l2,
                    upper: u2,
                },
            ) => l1 == l2 && u1 == u2,
            _ => false,
        }
    }
}

impl PartialOrd for AbstractInterval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Bottom, _) | (_, Self::Top) => Some(std::cmp::Ordering::Less),
            (_, Self::Bottom) | (Self::Top, _) => Some(std::cmp::Ordering::Greater),
            (
                Self::Bounded {
                    lower: l1,
                    upper: u1,
                },
                Self::Bounded {
                    lower: l2,
                    upper: u2,
                },
            ) => {
                if l1 >= l2 && u1 <= u2 {
                    Some(std::cmp::Ordering::Less) // `self` è contenuto in `other`
                } else if l1 <= l2 && u1 >= u2 {
                    Some(std::cmp::Ordering::Greater) // `other` è contenuto in `self`
                } else {
                    None
                }
            }
        }
    }
}

impl Add for AbstractInterval {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Bottom, _) | (_, Self::Bottom) => Self::Bottom,
            (Self::Top, _) | (_, Self::Top) => Self::Top,
            (
                Self::Bounded {
                    lower: l1,
                    upper: u1,
                },
                Self::Bounded {
                    lower: l2,
                    upper: u2,
                },
            ) => {
                let m = *M.lock().unwrap();
                let n = *N.lock().unwrap();
                if m == n {
                    let new_lower = m;
                    let new_upper = m;
                    Self::Bounded {
                        lower: new_lower,
                        upper: new_upper,
                    }
                } else {
                    let new_upper = checked_add(l1, l2);
                    let new_lower = checked_add(u1, u2);
                    if new_upper == n && new_lower == m {
                        return Self::Top;
                    }
                    // if new_upper == n || new_lower == m {
                    //     Self::Bottom
                    // } else {
                    if new_lower > new_upper {
                        Self::Bounded {
                            lower: new_upper,
                            upper: new_lower,
                        }
                    } else {
                        Self::Bounded {
                            lower: new_lower,
                            upper: new_upper,
                        }
                    }
                }
            }
        }
    }
}

fn checked_add(a: i64, b: i64) -> i64 {
    let m = *M.lock().unwrap();
    let n = *N.lock().unwrap();
    match a.checked_add(b) {
        Some(result) => {
            println!("some case in check add");
            if n >= result {
                if m <= result {
                    println!("normal case {}", result);
                    return result;
                }
                println!("returning min in check add");
                return m;
            }
            println!("returning max in check add");
            return n;
            //result
        }
        None if a > i64::zero() => {
            println!("none case returning max in check add");
            n
        }
        None => {
            println!("none case returning min in check add");
            m
        }
    }
}

impl Sub for AbstractInterval {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        println!("sub function call");
        match (self, other) {
            (Self::Bottom, _) | (_, Self::Bottom) => Self::Bottom,
            (Self::Top, _) | (_, Self::Top) => Self::Top,
            (
                Self::Bounded {
                    lower: l1,
                    upper: u1,
                },
                Self::Bounded {
                    lower: l2,
                    upper: u2,
                },
            ) => {
                let m = *M.lock().unwrap();
                let n = *N.lock().unwrap();
                if m == n {
                    let new_lower = m;
                    let new_upper = m;
                    Self::Bounded {
                        lower: new_lower,
                        upper: new_upper,
                    }
                } else {
                    println!("normal form of analysis");
                    // [a,b] [c,d] = [a-d, b-c]
                    let l1_pre = l1.clone();
                    let l2_pre = l2.clone();
                    let u1_pre = u1.clone();
                    let u2_pre = u2.clone();
                    let new_lower = checked_sub(l1, u2);
                    let new_upper = checked_sub(u1, l2);
                    println!("new upper in sub {}", new_upper);
                    println!("new lower in sub {}", new_lower);

                    if new_upper == n && new_lower == m {
                        println!("returning top in sub");
                        return Self::Top;
                    }

                    if l1_pre == m || l2_pre == m && u1_pre == n || u2_pre == n {
                        if new_upper == n || new_lower == m {
                            println!("returning bottom in sub");
                            Self::Bottom
                        } else {
                            println!("returning bounded in sub");
                            if new_lower > new_upper {
                                Self::Bounded {
                                    lower: new_upper,
                                    upper: new_lower,
                                }
                            } else {
                                Self::Bounded {
                                    lower: new_lower,
                                    upper: new_upper,
                                }
                            }
                        }
                    } else {
                        println!("returning bounded in sub");
                        if new_lower > new_upper {
                            Self::Bounded {
                                lower: new_upper,
                                upper: new_lower,
                            }
                        } else {
                            Self::Bounded {
                                lower: new_lower,
                                upper: new_upper,
                            }
                        }
                    }
                }
            }
        }
    }
}

fn checked_sub(a: i64, b: i64) -> i64 {
    println!("check sub function call");
    let m = *M.lock().unwrap();
    let n = *N.lock().unwrap();
    match a.checked_sub(b) {
        Some(result) => {
            println!("Some case in check sub");
            if n >= result {
                if m <= result {
                    println!("normal case {}", result);
                    return result;
                }
                println!("returning min");
                return m;
            }
            println!("returning max");
            return n;
        }
        None if a > i64::zero() => {
            println!("none case returnin max");
            n
        }
        None => {
            println!("none case returning min");
            m
        }
    }
}

impl Mul for AbstractInterval {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Bottom, _) | (_, Self::Bottom) => Self::Bottom,
            (Self::Top, _) | (_, Self::Top) => Self::Top,
            (
                Self::Bounded {
                    lower: l1,
                    upper: u1,
                },
                Self::Bounded {
                    lower: l2,
                    upper: u2,
                },
            ) => {
                let m = *M.lock().unwrap();
                let n = *N.lock().unwrap();
                if m == n {
                    let new_lower = m;
                    let new_upper = m;
                    Self::Bounded {
                        lower: new_lower,
                        upper: new_upper,
                    }
                } else {
                    let candidates = [
                        checked_mul(l1, l2),
                        checked_mul(l1, u2),
                        checked_mul(u1, l2),
                        checked_mul(u1, u2),
                    ];

                    let new_lower = candidates
                        .iter()
                        .filter_map(|&x| Some(x)) // Filtra i risultati validi
                        .min() // Trova il valore minimo
                        .unwrap_or(min_value()); // Se tutto fallisce, ritorna il valore minimo

                    let new_upper = candidates
                        .iter()
                        .filter_map(|&x| Some(x)) // Filtra i risultati validi
                        .max() // Trova il valore massimo
                        .unwrap_or(max_value()); // Se tutto fallisce, ritorna il valore massimo

                    Self::Bounded {
                        lower: new_lower,
                        upper: new_upper,
                    }
                }
            }
        }
    }
}

/// Funzione helper per effettuare una moltiplicazione controllata
fn checked_mul(a: i64, b: i64) -> i64 {
    let m = *M.lock().unwrap();
    let n = *N.lock().unwrap();
    match a.checked_mul(b) {
        Some(result) => {
            if n >= result {
                if m <= result {
                    return result;
                }
                return m;
            }
            return n;
            //result
        }
        None if a > i64::zero() => n,
        None => m,
    } // Usa il metodo built-in per tipi numerici nativi
}

/// Function to obtain the global values M (⊥) and N (⊤)
fn min_value() -> i64 {
    let m = *M.lock().unwrap();
    m
}

fn max_value() -> i64 {
    let n = *N.lock().unwrap();
    n
}
impl Div for AbstractInterval {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        let m = *M.lock().unwrap();
        let n = *N.lock().unwrap();
        if m == n {
            Self::Bounded { lower: m, upper: m }
        } else {
            match (self, other) {
                (Self::Bottom, _) | (_, Self::Bottom) => Self::Bottom, // ⊥ / qualsiasi = ⊥
                (Self::Top, Self::Top) => Self::Top,                   // ⊤ / ⊤ = ⊤
                (Self::Top, Self::Bounded { lower, upper }) if lower == 0 && upper == 0 => {
                    Self::Bottom
                } // ⊤ / [0,0] = ⊥ ✅
                (Self::Top, _) => Self::Top, // ⊤ / qualsiasi ≠ [0,0] = ⊤ ✅
                (_, Self::Top) => Self::Top, // qualsiasi / ⊤ = ⊤ ✅
                (
                    Self::Bounded {
                        lower: l1,
                        upper: u1,
                    },
                    Self::Bounded {
                        lower: l2,
                        upper: u2,
                    },
                ) => {
                    if l2 == 0 && u2 == 0 {
                        return Self::Bottom; // [l,u] / [0,0] = ⊥ ✅
                    }

                    if l2 < 0 && u2 > 0 {
                        return Self::Top; // [l,u] / [l',u'] con l' < 0 < u' = ⊤ ✅
                    }

                    let candidates = [
                        checked_div(l1, l2),
                        checked_div(l1, u2),
                        checked_div(u1, l2),
                        checked_div(u1, u2),
                    ];
                    let new_lower = candidates
                        .iter()
                        .filter_map(|&x| Some(x))
                        .min()
                        .unwrap_or(m);
                    let new_upper = candidates
                        .iter()
                        .filter_map(|&x| Some(x))
                        .max()
                        .unwrap_or(n);

                    Self::Bounded {
                        lower: new_lower,
                        upper: new_upper,
                    }
                }
            }
        }
    }
}
fn checked_div(a: i64, b: i64) -> i64 {
    let m = *M.lock().unwrap();
    let n = *N.lock().unwrap();
    match a.checked_div(b) {
        Some(result) => {
            if n >= result {
                if m <= result {
                    return result;
                }
                return m;
            }
            return n;
            // result
        }
        None if a > i64::zero() => n,
        None => m,
    } // Usa il metodo built-in per tipi numerici nativi
}

impl Neg for AbstractInterval {
    type Output = Self;

    fn neg(self) -> AbstractInterval {
        match self {
            AbstractInterval::Bottom => AbstractInterval::Bottom,
            AbstractInterval::Top => AbstractInterval::Top,
            AbstractInterval::Bounded { lower, upper } => {
                let m = *M.lock().unwrap();
                let n = *N.lock().unwrap();
                if m == n {
                    let new_lower = m;
                    let new_upper = m;
                    Self::Bounded {
                        lower: new_lower,
                        upper: new_upper,
                    }
                } else {
                    AbstractInterval::Bounded {
                        lower: -upper,
                        upper: -lower,
                    }
                }
            }
        }
    }
}

impl Ord for AbstractInterval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (
                AbstractInterval::Bounded {
                    lower: a1,
                    upper: b1,
                },
                AbstractInterval::Bounded {
                    lower: a2,
                    upper: b2,
                },
            ) => a1.cmp(a2).then(b1.cmp(b2)),
            // `Bottom` è sempre considerato più piccolo di qualsiasi altro intervallo
            (AbstractInterval::Bottom, _) => Ordering::Less,
            (_, AbstractInterval::Bottom) => Ordering::Greater,

            // `Top` è sempre considerato più grande di qualsiasi altro intervallo
            (AbstractInterval::Top, _) => Ordering::Greater,
            (_, AbstractInterval::Top) => Ordering::Less,
        }
    }
}
impl From<i64> for AbstractInterval {
    fn from(value: i64) -> Self {
        AbstractInterval::Bounded {
            lower: value,
            upper: value,
        }
    }
}
impl Zero for AbstractInterval {
    fn zero() -> Self {
        AbstractInterval::Bounded { lower: 0, upper: 0 }
    }

    fn is_zero(&self) -> bool {
        match self {
            AbstractInterval::Bounded { lower, upper } => *lower == 0 && *upper == 0,
            _ => false,
        }
    }
}
