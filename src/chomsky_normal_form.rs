use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::chomsky_production::ChomskyProduction;
use crate::model::Variable;

type Cell<'a> = HashSet<&'a char>;
type Row<'a> = Vec<Cell<'a>>;
type ChomskyTable<'a> = Vec<Row<'a>>;

pub struct ChomskyGrammar {
    start_symbol: char,
    rules: HashMap<char, Vec<ChomskyProduction>>,
}

impl ChomskyGrammar {
    /// checks which vars produce c as terminal...
    fn search_rules_producing_terminal(&self, terminal: &char) -> Cell {
        self.rules
            .iter()
            .filter(|(_, right)| right.iter().any(|p| p.produces_terminal(terminal)))
            .map(|(left, _)| left)
            .sorted()
            .collect::<HashSet<_>>()
    }

    fn search_rules_producing_sequence(&self, first_var: &char, second_var: &char) -> Cell {
        self.rules
            .iter()
            .filter(|(_, right)| right.iter().any(|p| p.produces_sequence(first_var, second_var)))
            .map(|(left, _)| left)
            .sorted()
            .collect::<HashSet<_>>()
    }

    fn build_table(&self, word: &[char]) -> ChomskyTable {
        let mut chomsky_table: ChomskyTable = Vec::with_capacity(word.len());

        // First row...
        chomsky_table.push(word.iter().map(|terminal| self.search_rules_producing_terminal(terminal)).collect::<Row>());

        for row_index in 1..word.len() {
            let next_row = (0..chomsky_table[row_index - 1].len() - 1)
                .map(|index| {
                    (0..row_index)
                        .flat_map(|run_index| {
                            let first_vars = &chomsky_table[run_index][index];
                            let second_vars = &chomsky_table[row_index - run_index - 1][index + run_index + 1];

                            first_vars.iter().cartesian_product(second_vars.iter())
                        })
                        .flat_map(|(first_var, second_var)| self.search_rules_producing_sequence(first_var, second_var))
                        .collect::<Cell>()
                })
                .collect::<Row>();

            chomsky_table.push(next_row);
        }

        chomsky_table
    }

    pub fn derive(&self, word: &[char]) -> bool {
        self.build_table(word)[word.len() - 1][0].contains(&self.start_symbol)
    }
}

#[cfg(test)]
mod tests {
    use crate::model::test_helpers::*;

    use super::*;

    fn seq(left: char, right: char) -> ChomskyProduction {
        ChomskyProduction::Sequence(Variable(left), Variable(right))
    }

    fn example_grammar() -> ChomskyGrammar {
        ChomskyGrammar {
            start_symbol: 'S',
            rules: HashMap::from([
                ('S', vec![seq('A', 'B')]),
                ('A', vec![seq('C', 'D'), seq('C', 'F')]),
                ('B', vec![ChomskyProduction::Terminal('c'), seq('E', 'B')]),
                ('C', vec![ChomskyProduction::Terminal('a')]),
                ('D', vec![ChomskyProduction::Terminal('b')]),
                ('E', vec![ChomskyProduction::Terminal('c')]),
                ('F', vec![seq('A', 'D')]),
            ]),
        }
    }

    fn chomsky_row<'a>(cells: &'a [&'a [char]]) -> Row<'a> {
        cells.iter().map(|cell| HashSet::from_iter(cell.iter())).collect::<Vec<_>>()
    }

    #[test]
    fn test_chomsky_production_produces() {
        assert!(ChomskyProduction::Terminal('a').produces_terminal(&'a'));
        assert!(ChomskyProduction::Terminal('a').produces_terminal(&'a'));

        assert!(!ChomskyProduction::Sequence(VAR_A, VAR_B).produces_terminal(&'a'));
    }

    #[test]
    fn test_build_table() {
        let grammar = example_grammar();

        let chomsky_table = grammar.build_table(&['a', 'a', 'a', 'b', 'b', 'b', 'c', 'c']);

        let awaited = vec![
            chomsky_row(&[&['C'], &['C'], &['C'], &['D'], &['D'], &['D'], &['B', 'E'], &['B', 'E']]),
            chomsky_row(&[&[], &[], &['A'], &[], &[], &[], &['B']]),
            chomsky_row(&[&[], &[], &['F'], &[], &[], &[]]),
            chomsky_row(&[&[], &['A'], &[], &[], &[]]),
            chomsky_row(&[&[], &['F'], &[], &[]]),
            chomsky_row(&[&['A'], &[], &[]]),
            chomsky_row(&[&['S'], &[]]),
            chomsky_row(&[&['S']]),
        ];

        assert_eq!(chomsky_table, awaited);
    }

    #[test]
    fn test_derive() {
        let grammar = example_grammar();

        assert!(grammar.derive(&['a', 'a', 'a', 'b', 'b', 'b', 'c', 'c']));
    }
}
