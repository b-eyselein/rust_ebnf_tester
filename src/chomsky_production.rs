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
