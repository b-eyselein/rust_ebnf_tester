use std::fmt::Formatter;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Terminal(pub char);

impl std::fmt::Display for Terminal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}'", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Variable(pub char);

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
#[allow(dead_code)]
pub mod test_helpers {
    use crate::model::*;

    pub const TER_0: Terminal = Terminal('0');
    pub const TER_1: Terminal = Terminal('1');

    pub const VAR_A: Variable = Variable('A');
    pub const VAR_B: Variable = Variable('B');
    pub const VAR_C: Variable = Variable('C');
    pub const VAR_D: Variable = Variable('D');
    pub const VAR_E: Variable = Variable('E');
    pub const VAR_F: Variable = Variable('F');
}

#[cfg(test)]
mod tests {
    use super::test_helpers::*;

    #[test]
    fn test_terminal() {
        assert_eq!(TER_1.to_string(), "'1'");
    }

    #[test]
    fn test_variable() {
        assert_eq!(VAR_A.to_string(), "A");
    }
}
