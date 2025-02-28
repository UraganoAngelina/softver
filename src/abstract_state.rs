use std::collections::HashMap;
use crate::abstract_interval::AbstractInterval;
use crate::abstract_domain::AbstractDomain;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct AbstractState {
    pub is_bottom: bool, // Bottom flag ⊥
    pub variables: HashMap<String, AbstractDomain<AbstractInterval>>,
}

impl PartialEq for AbstractDomain<AbstractInterval> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl AbstractState {
    // Builds an empty state (not ⊥)
    pub fn new() -> Self {
        Self {
            is_bottom: false,
            variables: HashMap::new(),
        }
    }
    fn is_top(&self) -> bool {
        // Se ci sono variabili, verificare se una di esse è Top
        self.variables.values().any(|interval| interval.value.is_top())
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
    pub fn update_interval(
        &mut self,
        variable_name: &str,
        new_interval: AbstractInterval,
    ) -> AbstractState {
        // Se lo stato è già bottom, restituire direttamente uno stato bottom
        if self.is_bottom() {
            return self.bottom();
        }

        // Recupera l'intervallo corrente della variabile, se esiste
        let current_domain = self
            .variables
            .get(variable_name)
            .cloned()
            .unwrap_or_else(|| AbstractDomain::new(AbstractInterval::Top));  // Top per default

        // Interseca l'intervallo corrente con quello nuovo
        let updated_interval = current_domain
            .get_value()
            .intersect(&new_interval);

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
    pub fn state_lub(&self, other: &AbstractState) -> AbstractState{
        if self.is_bottom {
            return other.clone();
        }
        if other.is_bottom {
            return self.clone();
        }

        let mut new_variables: HashMap<String , AbstractDomain<AbstractInterval>> = HashMap::new();

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
    
    // Widening operator for states
     pub fn state_widening(&self, other: &AbstractState) -> AbstractState {
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

        let mut new_variables: HashMap<String, AbstractDomain<AbstractInterval>> = HashMap::new();

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
   
    // Narrowing operator for states
    pub fn state_narrowing(&self, other: &AbstractState) -> AbstractState {
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
        let mut new_variables: HashMap<String, AbstractDomain<AbstractInterval>> = HashMap::new();

        for (key, left_interval) in &self.variables {
            if let Some(right_interval) = other.variables.get(key) {
                // Interval narrowing for every variable in both states
                new_variables.insert(key.clone(), left_interval.narrowing(&right_interval));
            } else {
                new_variables.insert(key.clone(), AbstractDomain::new(left_interval.value.clone()));
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

impl Clone for AbstractState {
    fn clone(&self) -> Self {
        Self {
            is_bottom: self.is_bottom,
            variables: self.variables.clone(),
        }
    }
}
