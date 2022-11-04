use crate::model::{Terminal, Variable};

#[derive(Debug, Clone, PartialEq)]
pub enum BackusNaurProduction {
    Epsilon,
    Terminal(Terminal),
    Variable(Variable),
    Optional(Box<BackusNaurProduction>),
    Alternative(Vec<BackusNaurProduction>),
    Sequence(Vec<BackusNaurProduction>),
}

#[cfg(test)]
impl BackusNaurProduction {
    pub fn terminal(c: char) -> Self {
        BackusNaurProduction::Terminal(Terminal(c))
    }

    pub fn variable(c: char) -> Self {
        BackusNaurProduction::Variable(Variable(c))
    }

    pub fn optional(sub_element: BackusNaurProduction) -> Self {
        BackusNaurProduction::Optional(Box::new(sub_element))
    }
}
