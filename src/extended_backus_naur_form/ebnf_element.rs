use std::fmt::Formatter;

use crate::model::{Terminal, Variable};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExtendedBackusNaurProduction {
    Epsilon,
    Terminal(Terminal),
    Variable(Variable),
    Optional(Box<ExtendedBackusNaurProduction>),
    RepetitionZero(Box<ExtendedBackusNaurProduction>),
    RepetitionOne(Box<ExtendedBackusNaurProduction>),
    Alternative(Vec<ExtendedBackusNaurProduction>),
    Sequence(Vec<ExtendedBackusNaurProduction>),
}

#[cfg(test)]
impl ExtendedBackusNaurProduction {
    pub fn terminal(c: char) -> Self {
        Self::Terminal(Terminal(c))
    }

    pub fn variable(c: char) -> Self {
        Self::Variable(Variable(c))
    }

    pub fn optional(sub_element: ExtendedBackusNaurProduction) -> Self {
        Self::Optional(Box::new(sub_element))
    }

    pub fn repetition_zero(sub_element: ExtendedBackusNaurProduction) -> Self {
        Self::RepetitionZero(Box::new(sub_element))
    }

    pub fn repetition_one(sub_element: ExtendedBackusNaurProduction) -> Self {
        Self::RepetitionOne(Box::new(sub_element))
    }
}

impl std::fmt::Display for ExtendedBackusNaurProduction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            ExtendedBackusNaurProduction::Epsilon => write!(f, "eps"),
            ExtendedBackusNaurProduction::Terminal(terminal) => write!(f, "{terminal}"),
            ExtendedBackusNaurProduction::Variable(variable) => write!(f, "{variable}"),
            ExtendedBackusNaurProduction::Optional(sub_element) => write!(f, "{sub_element}?"),
            ExtendedBackusNaurProduction::RepetitionZero(sub_element) => write!(f, "{sub_element}*"),
            ExtendedBackusNaurProduction::RepetitionOne(sub_element) => write!(f, "{sub_element}+"),
            ExtendedBackusNaurProduction::Alternative(alternatives) => {
                write!(f, "{}", alternatives.iter().map(|el| el.to_string()).collect::<Vec<_>>().join(" | "))
            }
            ExtendedBackusNaurProduction::Sequence(sub_elements) => write!(f, "{}", sub_elements.iter().map(|el| el.to_string()).collect::<Vec<_>>().join(" ")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TER_1: ExtendedBackusNaurProduction = ExtendedBackusNaurProduction::Terminal(Terminal('1'));

    const VAR_A: ExtendedBackusNaurProduction = ExtendedBackusNaurProduction::Variable(Variable('A'));
    const VAR_B: ExtendedBackusNaurProduction = ExtendedBackusNaurProduction::Variable(Variable('B'));
    const VAR_C: ExtendedBackusNaurProduction = ExtendedBackusNaurProduction::Variable(Variable('C'));

    #[test]
    fn test_rep_zero() {
        assert_eq!(ExtendedBackusNaurProduction::repetition_zero(VAR_A).to_string(), "A*");
    }

    #[test]
    fn test_rep_one() {
        assert_eq!(ExtendedBackusNaurProduction::repetition_one(VAR_A).to_string(), "A+");
    }

    #[test]
    fn test_option() {
        assert_eq!(ExtendedBackusNaurProduction::optional(VAR_A).to_string(), "A?");
    }

    #[test]
    fn test_alternative() {
        assert_eq!(ExtendedBackusNaurProduction::Alternative(vec![VAR_A, VAR_B]).to_string(), "A | B");

        assert_eq!(ExtendedBackusNaurProduction::Alternative(vec![VAR_A, VAR_B, VAR_C]).to_string(), "A | B | C");

        assert_eq!(ExtendedBackusNaurProduction::Alternative(vec![VAR_A, TER_1, VAR_C]).to_string(), "A | '1' | C");
    }

    #[test]
    fn test_sequence() {
        assert_eq!(ExtendedBackusNaurProduction::Sequence(vec![VAR_A, VAR_B]).to_string(), "A B");

        assert_eq!(ExtendedBackusNaurProduction::Sequence(vec![VAR_A, VAR_B, VAR_C]).to_string(), "A B C");

        assert_eq!(ExtendedBackusNaurProduction::Sequence(vec![VAR_A, TER_1, VAR_C]).to_string(), "A '1' C");
    }
}
