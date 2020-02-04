// imports

// [[file:~/Workspace/Programming/gchemol-rs/gchemol-core/gchemol-core.note::*imports][imports:1]]
pub use lattice::Lattice;

use crate::atom::Vector3f;
use crate::molecule::Molecule;
use guts::prelude::*;
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

    // FIXME: avoid type conversion
    /// Return fractional coordinates relative to unit cell. Return None if not
    /// a periodic structure
    pub fn scaled_positions(&self) -> Option<impl Iterator<Item = [f64; 3]> + '_> {
        self.lattice
            .map(|lat| self.positions().map(move |cart| lat.to_frac(cart).into()))
    }

    /// Set fractional coordinates relative to unit cell.
    ///
    /// Panics if Molecule is aperiodic structure.
    pub fn set_scaled_positions<T, P>(&mut self, scaled: T)
    where
        T: IntoIterator<Item = P>,
        P: Into<Vector3f>,
    {
        let mut lat = self
            .lattice
            .expect("cannot set scaled positions for aperiodic structure");
        let positions = scaled.into_iter().map(|frac| lat.to_cart(frac));
        self.set_positions(positions);
    }
}
// basic:1 ends here
