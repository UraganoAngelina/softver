use interval::interval::Interval;

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractInterval<T> {
    Bottom, // Rappresenta errore o inconsistenza
    Top,    // Assenza di informazioni
    Interval(Interval<T>),
}


impl<T> AbstractInterval<T>
where
    T: PartialOrd + Copy + interval::ops::Width,
{
    pub fn union(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Bottom, x) | (x, Self::Bottom) => x.clone(),
            (Self::Top, _) | (_, Self::Top) => Self::Top,
            (Self::Interval(a), Self::Interval(b)) => {
                Self::Interval(a.clone().union(b.clone()))
            }
        }
    }
}