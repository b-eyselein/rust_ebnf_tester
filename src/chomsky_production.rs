use crate::model::Variable;

pub enum ChomskyProduction {
    Terminal(char),
    Sequence(Variable, Variable),
}

impl ChomskyProduction {
    pub fn produces_terminal(&self, c: &char) -> bool {
        match self {
            ChomskyProduction::Terminal(t) => t == c,
            ChomskyProduction::Sequence(_, _) => false,
        }
    }

    pub fn produces_sequence(&self, var1: &char, var2: &char) -> bool {
        match self {
            ChomskyProduction::Terminal(_) => false,
            ChomskyProduction::Sequence(Variable(prod1), Variable(prod2)) => var1 == prod1 && var2 == prod2,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::test_helpers::*;

    use super::*;

    #[test]
    fn test_chomsky_production_produces() {
        assert!(ChomskyProduction::Terminal('a').produces_terminal(&'a'));
        assert!(ChomskyProduction::Terminal('a').produces_terminal(&'a'));

        assert!(!ChomskyProduction::Sequence(VAR_A, VAR_B).produces_terminal(&'a'));
    }
}
