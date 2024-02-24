// [[file:../gchemol-core.note::*header][header:1]]
//! A Trajectory represents a collection of molecules in the same size
//! but with different configurations.
// header:1 ends here

// [[file:../gchemol-core.note::a6e9e016][a6e9e016]]
use crate::lattice::Lattice;
use crate::molecule::Molecule;
use crate::Bond;
use crate::PropertyStore;

use crate::common::*;
// a6e9e016 ends here

// [[file:../gchemol-core.note::fabd768f][fabd768f]]
mod configuration {
    use super::*;

    /// Resprents the state of a molecule at a specfic frame in trajectory.
    #[derive(Debug, Clone, Deserialize, Serialize, Default)]
    pub struct Configuration {
        /// A descriptive message of this frame
        pub title: String,

        /// The molecule positions.
        pub(super) positions: Vec<[f64; 3]>,

        /// The lattice
        pub(super) lattice: Option<Lattice>,

        /// bonding connectivity?
        pub(super) bonds: Vec<(usize, usize, Bond)>,

        /// Configuration associated properties
        pub properties: PropertyStore,
    }

    impl Configuration {
        pub(super) fn from_molecule(mol: &Molecule) -> Self {
            Self {
                title: mol.title().to_owned(),
                positions: mol.positions().collect(),
                lattice: mol.lattice.clone(),
                properties: mol.properties.clone(),
                bonds: mol.bonds().map(|(u, v, b)| (u, v, b.clone())).collect(),
            }
        }

        pub(super) fn to_molecule(&self, mol: &Molecule) -> Molecule {
            let mut mol = mol.clone();
            mol.lattice = self.lattice.clone();
            mol.set_positions(self.positions.clone());
            mol.properties = self.properties.clone();
            mol
        }
    }

    impl Configuration {
        /// Update `parent` with current image state.
        pub(crate) fn update_molecule(&self, parent: &mut Molecule) {
            parent.set_positions(self.positions.to_owned());
            // parent.set_velocities(self.velocities.to_owned());
            parent.lattice = self.lattice.clone();
            parent.set_title(self.title.to_owned());
        }

        /// Set image positions
        pub(crate) fn set_positions(&mut self, positions: Vec<[f64; 3]>) {
            self.positions = positions;
        }
    }
}
// fabd768f ends here

// [[file:../gchemol-core.note::ce404893][ce404893]]
pub use self::configuration::Configuration;

/// A Trajectory represents a collection of molecules in the same size
/// but with different configurations.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Trajectory {
    /// Parent molecule of configuration frames in the trajectory.
    parent: Molecule,

    /// Frames represent molecular configuations along time axis.
    pub frames: Vec<Configuration>,
}
// ce404893 ends here

// [[file:../gchemol-core.note::a58ecc65][a58ecc65]]
fn matching(mol1: &Molecule, mol2: &Molecule) -> bool {
    // check number of atoms
    if mol1.natoms() == mol2.natoms() {
        // check elements
        let syms1 = mol1.symbols();
        let syms2 = mol2.symbols();
        if syms1.zip(syms2).all(|(s1, s2)| s1 == s2) {
            // check lattice
            let match_lat_none = mol1.lattice.is_none() && mol2.lattice.is_none();
            let match_lat_some = mol1.lattice.is_some() && mol2.lattice.is_some();
            return match_lat_none || match_lat_none;
        }
    }
    false
}
// a58ecc65 ends here

// [[file:../gchemol-core.note::c0387851][c0387851]]
impl Trajectory {
    /// Construct `Trajectory` object from a list of `Molecule`
    pub fn new(mols: Vec<Molecule>) -> Self {
        // FIXME: avoid re-allocation
        let frames: Vec<_> = mols.iter().map(Configuration::from_molecule).collect();

        Self {
            frames,
            parent: mols[0].clone(),
        }
    }

    /// Return true if trajectory has no frames.
    pub fn is_empty(&self) -> bool {
        self.frames.len() == 0
    }

    /// Return the number of frames in the trajectory.
    pub fn nframes(&self) -> usize {
        self.frames.len()
    }

    /// Return the number of atoms in each frame.
    pub fn natoms(&self) -> usize {
        self.parent.natoms()
    }

    /// Return an iterator over each `Molecule` in trajectory
    fn iter(&self) -> impl Iterator<Item = Molecule> + '_ {
        self.frames.iter().map(|conf| conf.to_molecule(&self.parent))
    }
}
// c0387851 ends here

// [[file:../gchemol-core.note::0a527b59][0a527b59]]
use std::convert::TryFrom;

impl TryFrom<Vec<Molecule>> for Trajectory {
    type Error = Error;

    fn try_from(mols: Vec<Molecule>) -> Result<Self> {
        for (i, pair) in mols.windows(2).enumerate() {
            if !matching(&pair[0], &pair[1]) {
                bail!("found inconsistent molecules: {} -- {}!", i, i + 1)
            }
        }
        Ok(Self::new(mols))
    }
}
// 0a527b59 ends here

// [[file:../gchemol-core.note::205c130c][205c130c]]
/// Trajectory related methods
impl Molecule {
    /// Test if matches `other` molecule for element types, irrespective geometric positions.
    pub fn matching_configuration(&self, other: &Molecule) -> bool {
        matching(self, other)
    }
}
// 205c130c ends here

// [[file:../gchemol-core.note::342ffb64][342ffb64]]
#[test]
fn test_trajectory() {
    let mol1 = Molecule::from_database("CH4");
    let mut mol2 = mol1.clone();
    let mut a1 = mol2.get_atom_mut(1).unwrap();
    a1.set_position([1.0, 2.0, 3.0]);
    assert!(mol2.matching_configuration(&mol1));

    let mut a2 = mol2.get_atom_mut(2).unwrap();
    a2.set_symbol("Fe");
    assert!(!mol2.matching_configuration(&mol1));

    let mut mol2 = mol1.clone();
    mol2.set_lattice(Lattice::default());
    assert!(!mol2.matching_configuration(&mol1));
}
// 342ffb64 ends here
