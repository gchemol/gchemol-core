// [[file:../gchemol-core.note::739e8c94][739e8c94]]
//! Calculate the molecular formula according to the Hill system order.
//!
//! https://web.stanford.edu/group/swain/cinf/workshop98aug/hill.html
// 739e8c94 ends here

// [[file:../gchemol-core.note::*imports][imports:1]]
use crate::molecule::Molecule;

use std::collections::HashMap;
use std::iter::IntoIterator;
// imports:1 ends here

// [[file:../gchemol-core.note::0d8318bb][0d8318bb]]
fn get_reduced_symbols<I>(symbols: I) -> HashMap<String, usize>
where
    I: IntoIterator,
    I::Item: std::fmt::Display,
{
    let symbols: Vec<_> = symbols.into_iter().map(|item| format!("{:}", item)).collect();

    // count symbols: CCCC ==> C 4
    let mut counts = HashMap::new();
    for x in symbols {
        let c = counts.entry(x).or_insert(0);
        *c += 1;
    }

    counts
}

fn get_reduced_formula<I>(symbols: I) -> String
where
    I: IntoIterator,
    I::Item: std::fmt::Display,
{
    let sym_and_counts = get_reduced_symbols(symbols);
    let mut syms: Vec<_> = sym_and_counts.keys().collect();
    syms.sort_by_key(|k| match k.as_str() {
        // Carbon first, Hydrogen second, and all remaining elements,
        // including Deuterium and Tritium, in alphabetical order.
        "C" => "0",
        "H" => "1",
        // If no Carbon is present, put all elements in alphabetical order.
        _ => k,
    });

    let s: Vec<_> = syms
        .into_iter()
        .map(|k| {
            let n = sym_and_counts[k];
            // omit number if the count is 1: C1H4 ==> CH4
            if n == 1 {
                k.to_string()
            } else {
                format!("{k}{n}")
            }
        })
        .collect();
    s.join("")
}

#[test]
fn test_formula() {
    let symbols = vec!["C", "H", "C", "H", "H", "H"];
    let formula = get_reduced_formula(&symbols);
    assert_eq!("C2H4", formula);
    let symbols = vec!["C", "H", "C", "H", "H", "O", "H", "O"];
    let formula = get_reduced_formula(&symbols);
    assert_eq!("C2H4O2", formula);

    let symbols = vec!["H", "H", "H", "O", "H", "O"];
    let formula = get_reduced_formula(&symbols);
    assert_eq!("H4O2", formula);
}
// 0d8318bb ends here

// [[file:../gchemol-core.note::3c178411][3c178411]]
/// Chemical formula
impl Molecule {
    /// Return the molecule formula represented in string according to
    /// the Hill system order. Return empty string if molecule
    /// containing no atom.
    pub fn formula(&self) -> String {
        get_reduced_formula(self.symbols())
    }

    /// Return a hashmap for counting atom symbols.
    pub fn reduced_symbols(&self) -> HashMap<String, usize> {
        get_reduced_symbols(self.symbols())
    }
}
// 3c178411 ends here
