// [[file:../gchemol-core.note::*imports][imports:1]]
use gut::prelude::*;
use serde::*;

use crate::{Atom, Bond, BondKind, Molecule};
// imports:1 ends here

// [[file:../gchemol-core.note::*impl/guess bonds][impl/guess bonds:1]]
#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct BondingConfig {
    /// Bonding ratio for guessing chemical bonds
    bonding_ratio: f64,
}

impl Default for BondingConfig {
    fn default() -> Self {
        // same value as mdanalysis
        Self { bonding_ratio: 0.55 }
    }
}

fn guess_bond(atom1: &Atom, atom2: &Atom, distance: f64) -> Bond {
    let config: BondingConfig = envy::prefixed("GCHEMOL_").from_env().unwrap_or_else(|e| {
        error!("parsing bonding env error: {:?}", e);
        BondingConfig::default()
    });

    match (atom1.get_vdw_radius(), atom2.get_vdw_radius()) {
        (Some(cr1), Some(cr2)) => {
            let r = config.bonding_ratio;
            let rcut = (cr1 + cr2) * r;
            if distance > rcut {
                Bond::dummy()
            } else {
                // FIXME: guess bond type
                Bond::single()
            }
        }
        _ => Bond::dummy(),
    }
}

// guess bonds in Jmol style
fn guess_bond_jmol(atom1: &Atom, atom2: &Atom, distance: f64) -> Bond {
    // JMOL: DEFAULT_BOND_TOLERANCE
    let bond_tolerance = 0.45;
    match (atom1.get_bonding_radius(), atom2.get_bonding_radius()) {
        (Some(r1), Some(r2)) => {
            let rcut = r1 + r2 + bond_tolerance;
            if distance > rcut {
                Bond::dummy()
            } else {
                Bond::single()
            }
        }
        _ => Bond::dummy(),
    }
}

/// Guess if bonds exist between two atoms based on their distance.
pub(crate) fn guess_bonds(mol: &Molecule) -> Vec<(usize, usize, Bond)> {
    // FIXME: lattice?
    let mut nh = neighbors::Neighborhood::new();
    let points = mol.atoms().map(|(i, atom)| (i, atom.position()));
    nh.update(points);

    let mut bonds = vec![];
    // NOTE: 1.6 is the largest cov radius of all elements (jmol)
    let d_bonding_cutoff = 1.6 * 2.0 + 0.4;
    for (i, atom_i) in mol.atoms() {
        let nns = nh.neighbors(i, d_bonding_cutoff);
        for n in nns {
            let j = n.node;
            // avoid double counting
            if i >= j {
                continue;
            }
            let atom_j = mol.get_atom(j).unwrap();
            // let bond = guess_bond(atom_i, atom_j, n.distance);
            let bond = guess_bond_jmol(atom_i, atom_j, n.distance);
            if !bond.is_dummy() {
                bonds.push((i, j, bond));
            }
        }
    }

    bonds
}
// impl/guess bonds:1 ends here

// [[file:../gchemol-core.note::*api][api:1]]
/// Handling chemical bonds in `Molecule`.
impl Molecule {
    /// Removes all existing bonds between atoms
    pub fn unbound(&mut self) {
        self.graph.clear_edges();
    }

    /// Removes all bonds between two selections to respect pymol's unbond command.
    ///
    /// Parameters
    /// ----------
    /// atom_indices1: the first collection of atoms
    ///
    /// atom_indices2: the other collection of atoms
    ///
    /// Reference
    /// ---------
    /// https://pymolwiki.org/index.php/Unbond
    ///
    pub fn unbond(&mut self, atom_indices1: &[usize], atom_indices2: &[usize]) {
        for &index1 in atom_indices1.iter() {
            for &index2 in atom_indices2.iter() {
                self.remove_bond(index1, index2);
            }
        }
    }

    /// Recalculates all bonds in molecule based on interatomic distances and
    /// covalent radii.
    pub fn rebond(&mut self) {
        if self.lattice.is_some() {
            // FIXME: impl
            warn!("rebond: treat as nonperiodic strucutre");
        }

        // remove all existing bonds
        self.unbound();
        let bonds = guess_bonds(&self);
        // add new bonds
        self.add_bonds_from(bonds);
    }
}
// api:1 ends here

// [[file:../gchemol-core.note::*test][test:1]]
#[test]
fn test_connect() {
    // CH4 molecule
    let atom1 = Atom::new("C", [-0.90203687, 0.62555259, 0.0081889]);
    let atom2 = Atom::new("H", [-0.54538244, -0.38325741, 0.0081889]);
    let atom3 = Atom::new("H", [-0.54536403, 1.12995078, 0.88184041]);
    let atom4 = Atom::new("H", [-0.54536403, 1.12995078, -0.8654626]);
    let atom5 = Atom::new("H", [-1.97203687, 0.62556577, 0.0081889]);

    let mut mol = Molecule::from_atoms(vec![atom1, atom2, atom3, atom4, atom5]);
    assert_eq!(mol.nbonds(), 0);
    mol.rebond();
    assert_eq!(mol.nbonds(), 4);
}
// test:1 ends here
