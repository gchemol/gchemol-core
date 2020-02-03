// imports

// [[file:~/Workspace/Programming/gchemol-rs/gchemol-core/gchemol-core.note::*imports][imports:1]]
use crate::molecule::Molecule;
use guts::prelude::*;
pub use ::lattice::Lattice;
// imports:1 ends here

// basic

// [[file:~/Workspace/Programming/gchemol-rs/gchemol-core/gchemol-core.note::*basic][basic:1]]
impl Molecule {
    /// Set periodic lattice
    pub fn set_lattice(&mut self, lat: Lattice) {
        self.lattice = Some(lat);
    }

    /// Unbuild current crystal structure leaving a nonperiodic structure
    pub fn unbuild_crystal(&mut self) {
        self.lattice = None
    }

    // /// Return fractional coordinates relative to unit cell.
    // /// Return None if not a periodic structure
    // pub fn scaled_positions(&self) -> Option<Vec<[f64; 3]>> {
    //     if let Some(mut lat) = self.lattice {
    //         let mut fxyzs = vec![];
    //         for (_, a) in self.atoms() {
    //             let xyz = a.position();
    //             let fxyz = lat.to_frac(xyz);
    //             fxyzs.push(fxyz.into())
    //         }
    //         Some(fxyzs)
    //     } else {
    //         None
    //     }
    // }

    // /// Set fractional coordinates relative to unit cell.
    // pub fn set_scaled_positions(&mut self, scaled: &[[f64; 3]]) -> Result<()> {
    //     if let Some(mut lat) = self.lattice {
    //         let mut positions = vec![];
    //         for &p in scaled {
    //             let xyz = lat.to_cart(p);
    //             positions.push(p);
    //         }

    //         self.set_positions(&positions)
    //     } else {
    //         bail!("cannot set scaled positions for aperiodic structure")
    //     }
    // }
}
// basic:1 ends here
