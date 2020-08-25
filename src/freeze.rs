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
    pub fn apply<T: Copy>(&self, out: &[T]) -> Vec<T> {
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

    /// Invert mask bit in-place.
    pub fn invert(&mut self) {
        for x in self.mask.iter_mut() {
            *x = !*x;
        }
    }

    /// Inverted `Mask`
    pub fn inverted(&self) -> Self {
        self.mask.iter().map(|x| !x).collect()
    }
}

impl FromIterator<bool> for Mask {
    fn from_iter<I: IntoIterator<Item = bool>>(iter: I) -> Self {
        Self {
            mask: iter.into_iter().collect(),
        }
    }
}

impl Molecule {
    /// Create a `Mask` for freezing atoms in 1-D vec.
    pub fn freezing_atoms_mask(&self) -> Mask {
        self.atoms().map(|(_, a)| a.is_fixed()).collect()
    }

    /// Create a `Mask` for Cartesian coordinates (3D) of freezing atoms in flatten 1-D vec.
    pub fn freezing_coords_mask(&self) -> Mask {
        self.atoms().flat_map(|(_, a)| a.freezing().to_vec().into_iter()).collect()
    }
}

// test

#[test]
fn test_mask() {
    let mut mask: Mask = vec![true, false, true, true].into_iter().collect();
    assert_eq!(mask.nmasked(), 3);
    mask.invert();
    assert_eq!(mask.nmasked(), 1);

    let mut mask_coords_fix: Mask = vec![true, false, true, true, false].into_iter().collect();
    let mask_coords_opt = mask_coords_fix.inverted();
    assert_eq!(mask_coords_opt.nmasked(), 2);
    let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let coords_opt_masked = mask_coords_opt.apply(&values);
    assert_eq!(coords_opt_masked.len(), 3);
}
