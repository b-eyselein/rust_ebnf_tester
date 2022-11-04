use crate::backus_naur_form::BackusNaurProduction;
use crate::extended_backus_naur_form::ebnf_element::ExtendedBackusNaurProduction;
use crate::extended_backus_naur_form::element_conversion_result::{ElementConversionResult, ElementsConversionResult};

pub mod ebnf_element;
mod element_conversion_result;

fn convert_child_elements(child_elements: Vec<ExtendedBackusNaurProduction>) -> ElementsConversionResult {
    child_elements
        .into_iter()
        .fold(ElementsConversionResult::empty(), |mut current_conversion_result, sub_el| {
            let ElementConversionResult {
                output_element,
                mut new_variables,
                new_replacers,
                mut new_rules,
            } = convert_to_bnf(sub_el);

            current_conversion_result.output_elements.push(output_element);
            current_conversion_result.new_variables.append(&mut new_variables);
            current_conversion_result.new_rules.append(&mut new_rules);

            for (key, value) in new_replacers {
                current_conversion_result.new_replacers.insert(key, value);
            }

            current_conversion_result
        })
}

pub fn convert_to_bnf(el: ExtendedBackusNaurProduction) -> ElementConversionResult {
    use ExtendedBackusNaurProduction::*;

    match el {
        Terminal(terminal) => ElementConversionResult::simple(BackusNaurProduction::Terminal(terminal)),
        Variable(variable) => ElementConversionResult::simple(BackusNaurProduction::Variable(variable)),
        Optional(_sub_element) => todo!(), // ElementConversionResult::simple(BackusNaurProduction::Optional(Box::new(convert_to_bnf(*sub_element)))),
        Alternative(_sub_elements) => todo!(), // ElementConversionResult::simple(BackusNaurProduction::Alternative(convert_child_elements(sub_elements))),
        Sequence(_sub_elements) => todo!(), // ElementConversionResult::simple(BackusNaurProduction::Sequence(convert_child_elements(sub_elements))),
        RepetitionZero(_sub_element) => todo!(),
        RepetitionOne(_sub_elements) => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::backus_naur_form::BackusNaurProduction as BnfProd;
    use crate::extended_backus_naur_form::ebnf_element::ExtendedBackusNaurProduction as EbnfProd;
    use crate::extended_backus_naur_form::element_conversion_result::ElementConversionResult;

    use super::*;

    #[test]
    fn test_convert_to_bnf() {
        let test_cases: Vec<(ExtendedBackusNaurProduction, ElementConversionResult)> = vec![
            (EbnfProd::terminal('1'), ElementConversionResult::simple(BnfProd::terminal('1'))),
            (EbnfProd::variable('A'), ElementConversionResult::simple(BnfProd::variable('A'))),
            (
                EbnfProd::optional(EbnfProd::variable('A')),
                ElementConversionResult::simple(BnfProd::optional(BnfProd::variable('A'))),
            ),
            (
                EbnfProd::Alternative(vec![EbnfProd::terminal('1'), EbnfProd::variable('A')]),
                ElementConversionResult::simple(BnfProd::Alternative(vec![BnfProd::terminal('1'), BnfProd::variable('A')])),
            ),
            (
                EbnfProd::Sequence(vec![EbnfProd::terminal('1'), EbnfProd::variable('A')]),
                ElementConversionResult::simple(BnfProd::Sequence(vec![BnfProd::terminal('1'), BnfProd::variable('A')])),
            ),
            (
                EbnfProd::repetition_zero(EbnfProd::terminal('1')),
                ElementConversionResult::simple(BnfProd::terminal('1')),
            ),
        ];

        for (ebnf_element, bnf_element) in test_cases {
            assert_eq!(convert_to_bnf(ebnf_element), bnf_element);
        }
    }
}
