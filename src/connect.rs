// [[file:../gchemol-core.note::*imports][imports:1]]
use gut::prelude::*;
use serde::*;

use crate::{Atom, Bond, BondKind, Molecule};
// imports:1 ends here

// [[file:../gchemol-core.note::270a1c57][270a1c57]]
#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct BondingConfig {
    /// Bonding ratio for guessing chemical bonds
    bonding_ratio: f64,
}

impl Default for BondingConfig {
    fn default() -> Self {
        // same value as mdanalysis
        Self {
            bonding_ratio: 0.55,
        }
    }
}

fn guess_bond(atom1: &Atom, atom2: &Atom, distance: f64) -> Bond {
    let config: BondingConfig = envy::prefixed("GCHEMOL_").from_env().unwrap_or_else(|e| {
        error!("parsing bonding env error: {:?}", e);
        BondingConfig::default()
    });

    match (atom1.get_vdw_radius(), atom2.get_vdw_radius()) {
        (Some(r1), Some(r2)) => {
            let r = config.bonding_ratio;
            let rcut = (r1 + r2) * r;
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
    // for molecule with periodic structure
    if let Some(lat) = mol.get_lattice() {
        nh.set_lattice(lat.matrix().into());
    }

    let mut bonds = vec![];
    // NOTE: 1.6 is the largest cov radius of all elements (jmol)
    let d_bonding_cutoff = 1.6 * 2.0 + 0.4;
    for (i, atom_i) in mol.atoms() {
        // unique neighbors of `i` no double counting
        let local_neighbors_uniq = nh.neighbors(i, d_bonding_cutoff).filter(|n| n.node > i);
        let mut connected = std::collections::HashMap::new();
        for n in local_neighbors_uniq {
            let j = n.node;
            // use minimum image convention for periodic structure: we only
            // count the bond between `i` and `j` calcualted with the nearest
            // image of atom `j`
            connected
                .entry(j)
                .and_modify(|dij| {
                    if n.distance < *dij {
                        *dij = n.distance;
                    }
                })
                .or_insert(n.distance);
        }
        for (j, dij) in connected {
            let atom_j = mol.get_atom(j).unwrap();
            let bond = guess_bond_jmol(atom_i, atom_j, dij);
            if !bond.is_dummy() {
                bonds.push((i, j, bond));
            }
        }
    }

    bonds
}
// 270a1c57 ends here

// [[file:../gchemol-core.note::96d22124][96d22124]]
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
    /// covalent radii. For periodic system, the bonds are determined by
    /// applying miniumu image convention.
    pub fn rebond(&mut self) {
        // remove all existing bonds
        self.unbound();
        let bonds = guess_bonds(&self);
        // add new bonds
        self.add_bonds_from(bonds);
    }
}
// 96d22124 ends here

// [[file:../gchemol-core.note::858392dd][858392dd]]
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

    let coords = "\
Si   0.000000     0.000000     0.000000
Si   0.000000     2.715350     2.715350
Si   2.715350     0.000000     2.715350
Si   2.715350     2.715350     0.000000
Si   4.073025     1.357675     4.073025
Si   1.357675     1.357675     1.357675
Si   1.357675     4.073025     4.073025
Si   4.073025     4.073025     1.357675";

    for line in coords.lines() {
        let atom: Atom = line.parse().unwrap();
    }
    let atoms: Vec<Atom> = coords
        .lines()
        .map(|line| line.parse().unwrap())
        .collect_vec();

    let mut mol = Molecule::from_atoms(atoms);
    let mut lat = crate::Lattice::from_params(5.430700, 5.430700, 5.430700, 90.0, 90.0, 90.0);
    mol.set_lattice(lat);
    assert_eq!(mol.natoms(), 8);
    assert!(mol.is_periodic());
    mol.rebond();
    assert_eq!(mol.nbonds(), 16);
    let connected = mol.connected(1).collect_vec();
    assert_eq!(connected.len(), 4);
    for x in [5, 6, 7, 8] {
        assert!(connected.contains(&x));
    }
}
// 858392dd ends here
