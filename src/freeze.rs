// imports

use crate::molecule::Molecule;

use gut::prelude::*;
use vecfx::*;

// impl

use std::iter::{FromIterator, IntoIterator};

/// A helper struct for masking/unmasking values in a vec.
#[derive(Debug, Clone)]
pub struct Mask {
    mask: Vec<bool>,
}

impl Mask {
    /// Recover the masked missing values with `fill_value`
    pub fn unmask(&self, inp: &[f64], fill_value: f64) -> Vec<f64> {
        let mask = &self.mask;

        assert!(inp.len() <= mask.len());
        let mut input_values = inp.into_iter().copied();

        mask.into_iter()
            .map(|&masked| {
                if masked {
                    fill_value
                } else {
                    input_values.next().expect("map inp")
                }
            })
            .collect()
    }

    /// Return a vec with masked values removed.
    pub fn apply(&self, out: &[f64]) -> Vec<f64> {
        let mask = &self.mask;
        assert!(out.len() == mask.len());

        out.into_iter()
            .zip(mask.into_iter())
            .filter_map(|(o, m)| if *m { None } else { Some(*o) })
            .collect()
    }

    /// Return number of masked values.
    pub fn nmasked(&self) -> usize {
        self.mask.iter().filter(|&x| *x).count()
    }
}

impl FromIterator<bool> for Mask {
    fn from_iter<I: IntoIterator<Item = bool>>(iter: I) -> Self {
        Self {
            mask: iter.into_iter().collect(),
        }
    }
}

/// Update positions of atoms in `mol` with `coords` exlucing freezing atoms.
fn update_coords_exclude_freezing_atoms(mol: &mut Molecule, coords: &[f64]) {
    let mask = mol.freezing_coords_mask();
    let positions = mask.unmask(coords, 0.0);
    mol.set_positions(positions.as_3d().to_vec());
}

/// Return a flat list of coordinates excluding any freezing atoms
fn get_coords_exclude_freezing_atoms(mol: &Molecule) -> Vec<f64> {
    let mask = mol.freezing_coords_mask();
    let out = mol.positions().collect_vec();
    mask.apply(out.as_flat())
}

impl Molecule {
    /// Create a `Mask` for freezing atoms in 1-D vec.
    pub fn freezing_atoms_mask(&self) -> Mask {
        self.atoms().map(|(_, a)| a.is_fixed()).collect()
    }

    /// Create a `Mask` for Cartesian coordinates (3D) of freezing atoms in flatten 1-D vec.
    pub fn freezing_coords_mask(&self) -> Mask {
        self.atoms()
            .flat_map(|(_, a)| a.freezing().to_vec().into_iter())
            .collect()
    }
}
