use std::collections::HashMap;
// use crate::abstract_interval::AbstractInterval;
use crate::abstract_domain::{AbstractDomain, AbstractDomainOps};
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
    fn _is_top(&self) -> bool {
        // Se ci sono variabili, verificare se una di esse è Top
        self.variables
            .values()
            .any(|interval| interval.value.is_top())
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
    // Least Upper Bound for states
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

    // Widening operator for states
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
    // Narrowing operator for states
    pub fn state_narrowing(&self, other: &AbstractState<Q>) -> AbstractState<Q> {
        if self.is_bottom {
            return other.clone();
        }
        if other.is_bottom {
            return self.clone();
        }

        // if self.is_top() || other.is_top() {
        //     return AbstractState {
        //         is_bottom: false,
        //         variables: HashMap::new(),
        //     };
        // }
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
}

impl<Q> fmt::Display for AbstractState<Q>
where
    Q: AbstractDomainOps + Clone + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("state flag: {}", self.is_bottom );
        if self.is_bottom {
            let variables_str: Vec<String> = self
                .variables
                .iter()
                .map(|(var, domain)| format!("{} : {:?}", var, domain))
                .collect();
            write!(f, "Bottom ⊥  {{ {} }}", variables_str.join(","))
        } else {
            let variables_str: Vec<String> = self
                .variables
                .iter()
                .map(|(var, domain)| format!("{}: {:?}", var, domain))
                .collect();
            write!(f, "{{ {} }}", variables_str.join(", "))
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
