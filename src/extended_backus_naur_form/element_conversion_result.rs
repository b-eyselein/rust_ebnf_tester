use std::collections::HashMap;

use crate::backus_naur_form::BackusNaurProduction;
use crate::extended_backus_naur_form::ebnf_element::ExtendedBackusNaurProduction;
use crate::model::Variable;

#[derive(Debug, PartialEq)]
pub struct ElementConversionResult<'a> {
    pub output_element: BackusNaurProduction,
    pub new_variables: Vec<char>,
    pub new_replacers: HashMap<&'a ExtendedBackusNaurProduction, Variable>,
    pub new_rules: Vec<(Variable, ExtendedBackusNaurProduction)>,
}

impl ElementConversionResult<'_> {
    pub fn simple(output_element: BackusNaurProduction) -> Self {
        Self {
            output_element,
            new_variables: Vec::new(),
            new_replacers: HashMap::new(),
            new_rules: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ElementsConversionResult<'a> {
    pub output_elements: Vec<BackusNaurProduction>,
    pub new_variables: Vec<char>,
    pub new_replacers: HashMap<&'a ExtendedBackusNaurProduction, Variable>,
    pub new_rules: Vec<(Variable, ExtendedBackusNaurProduction)>,
}

impl ElementsConversionResult<'_> {
    pub fn empty() -> Self {
        Self {
            output_elements: Vec::new(),
            new_variables: Vec::new(),
            new_replacers: HashMap::new(),
            new_rules: Vec::new(),
        }
    }
}
