// [[file:../gchemol-core.note::*imports][imports:1]]
pub use gchemol_lattice::Lattice;

use crate::atom::Vector3f;
use crate::molecule::Molecule;
// imports:1 ends here

// [[file:../gchemol-core.note::*api][api:1]]
/// Lattice related methods
impl Molecule {
    #[cfg(feature = "adhoc")]
    /// Get a reference to `Lattice` struct.
    pub fn get_lattice(&self) -> Option<&Lattice> {
        self.lattice.as_ref()
    }

    /// Set periodic lattice
    pub fn set_lattice(&mut self, lat: Lattice) {
        self.lattice = Some(lat);
    }

    /// Return true if `Molecule` is a periodic structure.
    pub fn is_periodic(&self) -> bool {
        self.lattice.is_some()
    }

    /// Unbuild current crystal structure leaving a nonperiodic structure
    pub fn unbuild_crystal(&mut self) {
        self.lattice = None
    }

    #[deprecated(note = "use get_scaled_positions instead")]
    /// Return fractional coordinates relative to unit cell. Return None if not
    /// a periodic structure
    pub fn scaled_positions(&self) -> Option<impl Iterator<Item = [f64; 3]> + '_> {
        self.get_scaled_positions()
    }

    // FIXME: avoid type conversion
    /// Return fractional coordinates relative to unit cell. Return None if not
    /// a periodic structure
    pub fn get_scaled_positions(&self) -> Option<impl Iterator<Item = [f64; 3]> + '_> {
        self.lattice
            .map(|lat| self.positions().map(move |cart| lat.to_frac(cart).into()))
    }

    /// Set fractional coordinates of atoms in sequence order.
    ///
    /// Panics if Molecule is aperiodic.
    pub fn set_scaled_positions<T, P>(&mut self, scaled: T)
    where
        T: IntoIterator<Item = P>,
        P: Into<Vector3f>,
    {
        let lat = self
            .lattice
            .expect("cannot set scaled positions for aperiodic structure");
        let positions = scaled.into_iter().map(|frac| lat.to_cart(frac));
        self.set_positions(positions);
    }

    /// Set fractional coordinates of atoms specified in serial numbers.
    ///
    /// Panics if Molecule is aperiodic.
    pub fn set_scaled_positions_from<T, P>(&mut self, scaled: T)
    where
        T: IntoIterator<Item = (usize, P)>,
        P: Into<Vector3f>,
    {
        let lat = self
            .lattice
            .expect("cannot set scaled positions for aperiodic structure");

        for (i, fi) in scaled {
            let pi = lat.to_cart(fi);
            self.set_position(i, pi);
        }
    }

    #[cfg(feature = "adhoc")]
    /// Create a supercell.
    ///
    /// # Arguments
    ///
    /// * sa, sb, sc: An sequence of three scaling factors. E.g., [2, 1, 1]
    /// specifies that the supercell should have dimensions 2a x b x c
    ///
    pub fn supercell(&self, sa: usize, sb: usize, sc: usize) -> Option<Molecule> {
        // add atoms by looping over three lattice directions
        let lat = self.lattice?;
        let (sa, sb, sc) = (sa as isize, sb as isize, sc as isize);
        let mut atoms = vec![];
        for image in lat.replicate(0..sa, 0..sb, 0..sc) {
            let mut m = self.clone();
            let t = lat.to_cart(image);
            m.translate(t);
            for (_, atom) in m.atoms() {
                atoms.push(atom.clone());
            }
        }
        let mut mol_new = Molecule::from_atoms(atoms);

        // update lattice
        let mut vabc = lat.vectors();
        let size = [sa, sb, sc];
        for v in vabc.iter_mut() {
            for i in 0..3 {
                v[i] *= size[i] as f64;
            }
        }

        mol_new.lattice = Some(Lattice::new(vabc));
        Some(mol_new)
    }
}
// api:1 ends here
