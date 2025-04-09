use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};
// use crate::abstract_interval::AbstractInterval;
use crate::{
    abstract_domain::{AbstractDomain, AbstractDomainOps, AbstractValue, ConcreteValue},
    abstract_interval::AbstractInterval,
};
use std::fmt::{self, Debug};

#[derive(Debug, PartialEq)]
pub struct AbstractState<Q: AbstractDomainOps + Clone> {
    pub is_bottom: bool, // Bottom flag ⊥
    pub variables: HashMap<String, AbstractDomain<Q>>,
}

impl<Q> PartialEq for AbstractDomain<Q>
where
    Q: std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<Q> AbstractState<Q>
where
    Q: AbstractDomainOps + Clone,
{
    // Builds an empty state (not ⊥)
    pub fn new() -> Self {
        Self {
            is_bottom: false,
            variables: HashMap::new(),
        }
    }
    // Checks if the state contains a Top interval
    fn _is_top(&self) -> bool {
        self.variables
            .values()
            .any(|interval| interval.value._is_top())
    }
    // Builds bottom state ⊥
    pub fn bottom(&self) -> AbstractState<Q> {
        AbstractState {
            is_bottom: true,
            variables: self.variables.clone(),
        }
    }
    // Checks if the state is ⊥
    pub fn is_bottom(&self) -> bool {
        if self.is_bottom {
            return true;
        }
        for domain in self.variables.values() {
            if domain.is_bottom() {
                return true;
            }
        }
        false
    }
    // Updates a specific interval in the state
    pub fn update_interval(&mut self, variable_name: &str, new_interval: Q) -> AbstractState<Q> {
        // Se lo stato è già bottom, restituire direttamente uno stato bottom
        if self.is_bottom() {
            return self.bottom();
        }

        // Recupera l'intervallo corrente della variabile, se esiste
        let current_domain = self
            .variables
            .get(variable_name)
            .cloned()
            .unwrap_or_else(|| AbstractDomain::new(Q::top())); // Top per default

        // Interseca l'intervallo corrente con quello nuovo
        let updated_interval = current_domain.get_value().glb(&new_interval);

        // Se il risultato è Bottom, impostare lo stato a bottom
        if updated_interval.is_bottom() {
            return self.bottom();
        }

        // Aggiorna lo stato con il nuovo intervallo
        self.variables.insert(
            variable_name.to_string(),
            AbstractDomain::new(updated_interval),
        );

        // Restituisce lo stato aggiornato
        self.clone()
    }
    // Least Upper Bound variable wise
    pub fn state_lub(&self, other: &AbstractState<Q>) -> AbstractState<Q> {
        if self.is_bottom {
            return other.clone();
        }
        if other.is_bottom {
            return self.clone();
        }

        let mut new_variables: HashMap<String, AbstractDomain<Q>> = HashMap::new();

        // Doing the State Lub
        for (key, left_domain) in &self.variables {
            if let Some(right_domain) = other.variables.get(key) {
                // Interval Lub for every variable
                new_variables.insert(
                    key.clone(),
                    AbstractDomain::new(left_domain.lub(right_domain).value),
                );
            } else {
                new_variables.insert(key.clone(), AbstractDomain::new(left_domain.value.clone()));
            }
        }

        for (key, right_domain) in &other.variables {
            if !self.variables.contains_key(key) {
                new_variables.insert(key.clone(), AbstractDomain::new(right_domain.value.clone()));
            }
        }

        AbstractState {
            is_bottom: false,
            variables: new_variables,
        }
    }
    pub fn state_glb(&self, other: &AbstractState<Q>) -> AbstractState<Q> {
        if self.is_bottom {
            return other.clone();
        }
        if other.is_bottom {
            return self.clone();
        }

        let mut new_variables: HashMap<String, AbstractDomain<Q>> = HashMap::new();

        // Doing the State Lub
        for (key, left_domain) in &self.variables {
            if let Some(right_domain) = other.variables.get(key) {
                // Interval Lub for every variable
                new_variables.insert(
                    key.clone(),
                    AbstractDomain::new(left_domain.glb(right_domain).value),
                );
            } else {
                new_variables.insert(key.clone(), AbstractDomain::new(left_domain.value.clone()));
            }
        }

        for (key, right_domain) in &other.variables {
            if !self.variables.contains_key(key) {
                new_variables.insert(key.clone(), AbstractDomain::new(right_domain.value.clone()));
            }
        }

        AbstractState {
            is_bottom: false,
            variables: new_variables,
        }
    }
    // Widening operator variable wise
    pub fn state_widening(&self, other: &AbstractState<Q>) -> AbstractState<Q> {
        // Se uno dei due stati è Bottom, ritorna l'altro stato
        if self.is_bottom {
            return other.clone();
        }
        if other.is_bottom {
            return self.clone();
        }

        // if self.is_top(){
        //     return AbstractState{
        //         is_bottom: false,
        //         variables:
        //     }
        // }
        // if self.is_top() || other.is_top() {
        //     return AbstractState {
        //         is_bottom: false,
        //         variables: HashMap::new(),
        //     };
        // }

        let mut new_variables: HashMap<String, AbstractDomain<Q>> = HashMap::new();

        for (key, left_interval) in &self.variables {
            if let Some(right_interval) = other.variables.get(key) {
                // Interval widening for every variable in both states
                new_variables.insert(key.clone(), left_interval.widening(right_interval));
            } else {
                new_variables.insert(key.clone(), left_interval.clone());
            }
        }

        for (key, right_interval) in &other.variables {
            if !self.variables.contains_key(key) {
                new_variables.insert(key.clone(), right_interval.clone());
            }
        }

        let newstate = AbstractState {
            is_bottom: false,
            variables: new_variables,
        };
        newstate
    }
    pub fn _lookup(&mut self, var: &str) -> AbstractDomain<Q> {
        let res = self
            .variables
            .get(var)
            .cloned()
            .expect("error in the lookup state function");
        res
    }
    // Narrowing operator variable wise
    pub fn state_narrowing(&self, other: &AbstractState<Q>) -> AbstractState<Q> {
        if self.is_bottom {
            return other.clone();
        }
        if other.is_bottom {
            return self.clone();
        }

        let mut new_variables: HashMap<String, AbstractDomain<Q>> = HashMap::new();

        for (key, left_interval) in &self.variables {
            if let Some(right_interval) = other.variables.get(key) {
                // Interval narrowing for every variable in both states
                new_variables.insert(key.clone(), left_interval.narrowing(&right_interval));
            } else {
                new_variables.insert(
                    key.clone(),
                    AbstractDomain::new(left_interval.value.clone()),
                );
            }
        }

        for (key, right_interval) in &other.variables {
            if !self.variables.contains_key(key) {
                new_variables.insert(key.clone(), right_interval.clone());
            }
        }

        let newstate = AbstractState {
            is_bottom: false, // Lo stato risultante non è Bottom
            variables: new_variables,
        };
        newstate
    }
    // Concretization function variable wise
    pub fn _state_gamma(&self) -> HashSet<ConcreteValue> {
        if self.is_bottom() {
            return HashSet::new();
        }
        let mut result = HashSet::new();

        for element in self.variables.values() {
            result.extend(element._gamma());
        }
        result
    }
    // Abstraction function variable wise
    pub fn _state_alpha(r: HashSet<ConcreteValue>) -> HashSet<AbstractValue> {
        <AbstractInterval as AbstractDomainOps>::_alpha(r)
    }
}

impl<Q> fmt::Display for AbstractState<Q>
where
    Q: AbstractDomainOps
        + Clone
        + fmt::Display
        + Copy
        + Ord
        + From<i64>
        + num_traits::Zero
        + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // println!("state flag: {}", self.is_bottom );
        if self.is_bottom {
            let variables_str: Vec<String> = self
                .variables
                .iter()
                .map(|(var, domain)| format!("{}: {}", var, domain))
                .collect();
            write!(f, "Bottom ⊥  {{{}}}", variables_str.join(", "))
        } else {
            let variables_str: Vec<String> = self
                .variables
                .iter()
                .map(|(var, domain)| format!("{}: {}", var, domain))
                .collect();
            write!(f, "{{{}}}", variables_str.join(", "))
        }
    }
}

impl<Q> Clone for AbstractState<Q>
where
    Q: AbstractDomainOps + Clone,
{
    fn clone(&self) -> Self {
        Self {
            is_bottom: self.is_bottom,
            variables: self.variables.clone(),
        }
    }
}
// partial order implementation variable wise
impl<Q: AbstractDomainOps + Clone + PartialOrd + PartialEq> PartialOrd for AbstractState<Q> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Bottom case
        if self.is_bottom() && other.is_bottom() {
            return Some(Ordering::Equal);
        }
        if self.is_bottom() {
            return Some(Ordering::Less);
        }
        if other.is_bottom() {
            return Some(Ordering::Greater);
        }

        let keys: HashSet<&String> = self
            .variables
            .keys()
            .chain(other.variables.keys())
            .collect();

        let mut all_leq = true;
        let mut all_geq = true;

        for key in keys {
            match (self.variables.get(key), other.variables.get(key)) {
                (Some(self_dom), Some(other_dom)) => {
                    if self_dom.value > other_dom.value {
                        all_leq = false;
                    }
                    if self_dom.value < other_dom.value {
                        all_geq = false;
                    }
                }
                _ => return None,
            }
        }

        if all_leq && all_geq {
            Some(Ordering::Equal)
        } else if all_leq {
            Some(Ordering::Less)
        } else if all_geq {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}
