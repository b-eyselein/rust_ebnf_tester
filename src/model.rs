use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq)]
pub enum EbnfElement {
    Terminal(char),
    Variable(char),
    Optional(Box<EbnfElement>),
    RepetitionZero(Box<EbnfElement>),
    RepetitionOne(Box<EbnfElement>),
    Alternative(Vec<EbnfElement>),
    Sequence(Vec<EbnfElement>),
}

#[cfg(test)]
impl EbnfElement {
    pub fn optional(sub_element: EbnfElement) -> Self {
        Self::Optional(Box::new(sub_element))
    }

    pub fn repetition_zero(sub_element: EbnfElement) -> Self {
        Self::RepetitionZero(Box::new(sub_element))
    }

    pub fn repetition_one(sub_element: EbnfElement) -> Self {
        Self::RepetitionOne(Box::new(sub_element))
    }
}

impl std::fmt::Display for EbnfElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            EbnfElement::Terminal(terminal) => write!(f, "'{terminal}'"),
            EbnfElement::Variable(variable) => write!(f, "{variable}"),
            EbnfElement::Optional(sub_element) => write!(f, "{sub_element}?"),
            EbnfElement::RepetitionZero(sub_element) => write!(f, "{sub_element}*"),
            EbnfElement::RepetitionOne(sub_element) => write!(f, "{sub_element}+"),
            EbnfElement::Alternative(alternatives) => write!(f, "{}", alternatives.iter().map(|el| el.to_string()).collect::<Vec<_>>().join(" | ")),
            EbnfElement::Sequence(sub_elements) => write!(f, "{}", sub_elements.iter().map(|el| el.to_string()).collect::<Vec<_>>().join(" ")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TER_1: EbnfElement = EbnfElement::Terminal('1');

    const VAR_A: EbnfElement = EbnfElement::Variable('A');
    const VAR_B: EbnfElement = EbnfElement::Variable('B');
    const VAR_C: EbnfElement = EbnfElement::Variable('C');

    #[test]
    fn test_terminal() {
        assert_eq!(TER_1.to_string(), "'1'");
    }

    #[test]
    fn test_variable() {
        assert_eq!(VAR_A.to_string(), "A");
    }

    #[test]
    fn test_rep_zero() {
        assert_eq!(EbnfElement::repetition_zero(VAR_A).to_string(), "A*");
    }

    #[test]
    fn test_rep_one() {
        assert_eq!(EbnfElement::repetition_one(VAR_A).to_string(), "A+");
    }

    #[test]
    fn test_option() {
        assert_eq!(EbnfElement::optional(VAR_A).to_string(), "A?");
    }

    #[test]
    fn test_alternative() {
        assert_eq!(EbnfElement::Alternative(vec![VAR_A, VAR_B]).to_string(), "A | B");

        assert_eq!(EbnfElement::Alternative(vec![VAR_A, VAR_B, VAR_C]).to_string(), "A | B | C");

        assert_eq!(EbnfElement::Alternative(vec![VAR_A, TER_1, VAR_C]).to_string(), "A | '1' | C");
    }

    #[test]
    fn test_sequence() {
        assert_eq!(EbnfElement::Sequence(vec![VAR_A, VAR_B]).to_string(), "A B");

        assert_eq!(EbnfElement::Sequence(vec![VAR_A, VAR_B, VAR_C]).to_string(), "A B C");

        assert_eq!(EbnfElement::Sequence(vec![VAR_A, TER_1, VAR_C]).to_string(), "A '1' C");
    }
}
