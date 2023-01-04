// [[file:../gchemol-core.note::4c6fdd8b][4c6fdd8b]]
use crate::common::*;
use crate::{Atom, Molecule};

use std::collections::HashSet;
// 4c6fdd8b ends here

// [[file:../gchemol-core.note::d5924a86][d5924a86]]
/// Select atoms by expanding `n` chemical bonds away from the center atom `m`
fn select_atoms_by_expanding_bond_(mol: &Molecule, m: usize, n: usize) -> HashSet<usize> {
    match n {
        0 => vec![m].into_iter().collect(),
        _ => {
            let mut selection: HashSet<_> = vec![m].into_iter().collect();
            for o in mol.connected(m) {
                let selected = select_atoms_by_expanding_bond_(mol, o, n - 1);
                selection.extend(selected);
            }
            selection
        }
    }
}
// d5924a86 ends here

// [[file:../gchemol-core.note::90d8094c][90d8094c]]
impl Molecule {
    /// Return selected atoms by expanding `n` chemical bonds away from
    /// the center atom `m`
    ///
    /// Note: the center atom m is put last in returned molecule.
    pub fn selection_by_expanding_bond(&self, m: usize, n: usize) -> Vec<usize> {
        let mut nodes = select_atoms_by_expanding_bond_(&self, m, n);
        // make sure central atom is the last one
        assert!(nodes.remove(&m));
        let mut nodes: Vec<_> = nodes.into_iter().collect();
        nodes.push(m);
        nodes
    }

    /// Return selected atoms by cutoff distance `r` nearby central atom `n`
    pub fn selection_by_distance(&self, n: usize, r: f64) -> Vec<usize> {
        assert!(r.is_sign_positive(), "invalid cutoff distance {r:?}");
        todo!();
    }
}
// 90d8094c ends here
