use std::collections::HashMap;
use std::fmt;

use crate::abstract_domain::AbstractInterval;

#[derive(Debug, PartialEq)]
pub struct AbstractState {
    pub is_bottom: bool,                                   // Bottom flag ⊥
    pub variables: HashMap<String, AbstractInterval<i64>>, // Abstract Variables
}

impl AbstractState {
    // Builds an empty state (not ⊥)
    pub fn new() -> Self {
        Self {
            is_bottom: false,
            variables: HashMap::new(),
        }
    }
    // Builds bottom state ⊥
    pub fn bottom(&self) -> AbstractState {
        AbstractState {
            is_bottom: true,
            variables: self.variables.clone(),
        }
    }
    // Checks if the state is ⊥
    pub fn is_bottom(&self) -> bool {
        self.is_bottom
    }
    // Updates a specific interval in the state
    pub fn update_interval(&mut self, variable_name: &str, new_interval: AbstractInterval<i64>) -> AbstractState {
        // Se lo stato è già bottom, restituire direttamente uno stato bottom
        if self.is_bottom() {
            return self.bottom();
        }

        // Recupera l'intervallo corrente della variabile, se esiste
        let current_interval = self
            .variables
            .get(variable_name)
            .cloned()
            .unwrap_or(AbstractInterval::Top);

        // Interseca l'intervallo corrente con quello nuovo
        let updated_interval = current_interval.intersect(&new_interval);

        // Se il risultato è Bottom, impostare lo stato a bottom
        if updated_interval.is_bottom() {
            return self.bottom();
        }

        // Aggiorna lo stato con il nuovo intervallo
        self.variables
            .insert(variable_name.to_string(), updated_interval);

        // Restituisce lo stato aggiornato
        self.clone()
    }
    // Checks if the state is ⊤
    fn is_top(&self) -> bool {
        // Se ci sono variabili, verificare se una di esse è Top
        self.variables.values().any(|interval| interval.is_top())
    }
    // Least Upper Bound for states
    pub fn state_lub(&self, other: &AbstractState) -> AbstractState {
        // One is bottom, return the other one
        if self.is_bottom {
            return other.clone();
        }
        if other.is_bottom {
            return self.clone();
        }

        // Both Top, return Top
        if self.is_top() || other.is_top() {
            return AbstractState {
                is_bottom: false,
                variables: HashMap::new(), 
            };
        }

        let mut new_variables: HashMap<String, AbstractInterval<i64>> = HashMap::new();

        // Doing the State Lub
        for (key, left_interval) in &self.variables {
            if let Some(right_interval) = other.variables.get(key) {
                // Interval Lub for every variable
                new_variables.insert(key.clone(), left_interval.int_lub(right_interval));
            } else {
                // Adding variables that are only in first state
                new_variables.insert(key.clone(), left_interval.clone());
            }
        }

        // Adding variables that are only in the second state
        for (key, right_interval) in &other.variables {
            if !self.variables.contains_key(key) {
                new_variables.insert(key.clone(), right_interval.clone());
            }
        }

        // New state creation
        let newstate = AbstractState {
            is_bottom: false, 
            variables: new_variables,
        };
        newstate
    }
    // Widening operator for states
    pub fn state_widening(&self, other: &AbstractState) -> AbstractState {
        // Se uno dei due stati è Bottom, ritorna l'altro stato
        if self.is_bottom {
            return other.clone();
        }
        if other.is_bottom {
            return self.clone();
        }

        if self.is_top() || other.is_top() {
            return AbstractState {
                is_bottom: false,
                variables: HashMap::new(),
            };
        }

        let mut new_variables: HashMap<String, AbstractInterval<i64>> = HashMap::new();

        for (key, left_interval) in &self.variables {
            if let Some(right_interval) = other.variables.get(key) {
                // Interval widening for every variable in both states
                new_variables.insert(key.clone(), left_interval.int_widening(right_interval));
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
    // Narrowing operator for states
    pub fn state_narrowing(&self, other: &AbstractState) -> AbstractState {
        if self.is_bottom {
            return other.clone();
        }
        if other.is_bottom {
            return self.clone();
        }

        if self.is_top() || other.is_top() {
            return AbstractState {
                is_bottom: false,
                variables: HashMap::new(),
            };
        }

        let mut new_variables: HashMap<String, AbstractInterval<i64>> = HashMap::new();

        for (key, left_interval) in &self.variables {
            if let Some(right_interval) = other.variables.get(key) {
                // Interval narrowing for every variable in both states
                new_variables.insert(key.clone(), left_interval.int_narrowing(right_interval));
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
            is_bottom: false, // Lo stato risultante non è Bottom
            variables: new_variables,
        };
        newstate
    }
}

impl fmt::Display for AbstractState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_bottom {
            // Stato è ⊥ (bottom)
            write!(f, "Bottom ⊥")
        } else {
            // Stato normale: stampiamo le variabili con i loro intervalli
            let variables_str: Vec<String> = self
                .variables
                .iter()
                .map(|(var, interval)| format!("{}: {}", var, interval))
                .collect();
            write!(f, "{{ {} }}", variables_str.join(", "))
        }
    }
}

impl Clone for AbstractState {
    fn clone(&self) -> Self {
        Self {
            is_bottom: self.is_bottom,
            variables: self.variables.clone(),
        }
    }
}
