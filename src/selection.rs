// [[file:../gchemol-core.note::4c6fdd8b][4c6fdd8b]]
use crate::common::*;
use crate::{Atom, Molecule};

use std::collections::HashSet;

use neighbors::{Neighbor, Neighborhood};
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

// [[file:../gchemol-core.note::a95d3144][a95d3144]]
/// Return a `Neighborhood` struct for probing nearest neighbors in `mol`
///
/// N.B. The neighbor node index is defined using atom serial number
fn create_neighborhood_probe(mol: &Molecule) -> Neighborhood {
    let particles: Vec<_> = mol.atoms().map(|(i, a)| (i, a.position())).collect();
    let mut nh = Neighborhood::new();
    nh.update(particles);
    if let Some(lat) = mol.lattice {
        nh.set_lattice(lat.matrix().into());
    }

    nh
}
// a95d3144 ends here

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

    /// Return selected atoms by cutoff distance `r` nearby central
    /// atom `n` NOTE: For periodic structure, the selection returned
    /// could be not unique due to periodic images.
    pub fn selection_by_distance(&self, n: usize, r: f64) -> Vec<usize> {
        assert!(r.is_sign_positive(), "invalid cutoff distance {r:?}");

        // FIXME: periodic images?
        let nh = create_neighborhood_probe(self);
        nh.neighbors(n, r).map(|n| n.node).chain(Some(n)).collect()
    }

    /// Return a `Neighborhood` struct for probing nearest neighbors in `mol`
    ///
    /// N.B. The neighbor node index is defined using atom serial number
    ///
    /// # Example
    ///
    /// ```rust, ignore, no_run
    /// let probe = mol.create_neighbor_probe();
    /// let p = [1.0, 2.0, 3.0];
    /// let r_cut = 3.2;
    /// let neighbors = probe.search(p, r_cut);
    /// let neighbors = probe.neighbors(12, r_cut);
    /// ```
    pub fn create_neighbor_probe(&self) -> Neighborhood {
        create_neighborhood_probe(&self)
    }
}
// 90d8094c ends here
