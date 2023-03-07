// [[file:../gchemol-core.note::735b5f39][735b5f39]]
pub use gchemol_lattice::Lattice;

use crate::atom::Vector3f;
use crate::common::*;
use crate::molecule::Molecule;
// 735b5f39 ends here

// [[file:../gchemol-core.note::a86668b2][a86668b2]]
/// Lattice related methods
impl Molecule {
    #[cfg(feature = "adhoc")]
    /// Get a reference to `Lattice` struct.
    pub fn get_lattice(&self) -> Option<&Lattice> {
        self.lattice.as_ref()
    }

    #[cfg(feature = "adhoc")]
    /// Get a mutable reference to `Lattice` struct.
    pub fn get_lattice_mut(&mut self) -> Option<&mut Lattice> {
        self.lattice.as_mut()
    }

    /// Set periodic lattice
    pub fn set_lattice(&mut self, lat: Lattice) {
        self.lattice = Some(lat);
    }

    /// Return true if `Molecule` is a periodic structure.
    pub fn is_periodic(&self) -> bool {
        self.lattice.is_some()
    }

    /// Unbuild current crystal structure leaving a nonperiodic
    /// structure. Return `Lattice` object if periodic, otherwise return None.
    pub fn unbuild_crystal(&mut self) -> Option<Lattice> {
        self.lattice.take()
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
        self.lattice.map(|lat| self.positions().map(move |cart| lat.to_frac(cart).into()))
    }

    /// Set fractional coordinates of atoms in sequence order.
    ///
    /// Panics if Molecule is aperiodic.
    pub fn set_scaled_positions<T, P>(&mut self, scaled: T)
    where
        T: IntoIterator<Item = P>,
        P: Into<Vector3f>,
    {
        let lat = self.lattice.expect("cannot set scaled positions for aperiodic structure");
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
        let lat = self.lattice.expect("cannot set scaled positions for aperiodic structure");

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
        mol_new.name = self.name.to_string();

        mol_new.lattice = Some(Lattice::new(vabc));
        Some(mol_new)
    }
}
// a86668b2 ends here

// [[file:../gchemol-core.note::599d9ac9][599d9ac9]]
impl Molecule {
    #[cfg(feature = "adhoc")]
    /// Create a `Lattice` from the minimal bounding box of the `Molecule`
    /// extended by a positive value of `padding`. NOTE: padding has to be large
    /// enough (> 0.5) to avoid self interaction with its periodic mirror.
    pub fn set_lattice_from_bounding_box(&mut self, padding: f64) {
        self.recenter();
        let [a, b, c] = self.bounding_box(padding);
        let center = [a / 2.0, b / 2.0, c / 2.0];
        self.translate(center);
        let mat = [[a, 0.0, 0.0], [0.0, b, 0.0], [0.0, 0.0, c]];
        let lat = Lattice::new(mat);
        self.set_lattice(lat);
    }

    /// Return minimal bounding box in x, y, z directions
    pub fn bounding_box(&self, padding: f64) -> [f64; 3] {
        use vecfx::*;

        assert!(padding.is_sign_positive(), "invalid scale factor: {padding}");
        let xmax = self.positions().map(|[x, _, _]| x).float_max();
        let xmin = self.positions().map(|[x, _, _]| x).float_min();
        let ymax = self.positions().map(|[_, y, _]| y).float_max();
        let ymin = self.positions().map(|[_, y, _]| y).float_min();
        let zmax = self.positions().map(|[_, _, z]| z).float_max();
        let zmin = self.positions().map(|[_, _, z]| z).float_min();

        let a = xmax - xmin + 2.0 * padding;
        let b = ymax - ymin + 2.0 * padding;
        let c = zmax - zmin + 2.0 * padding;
        [a, b, c]
    }
}
// 599d9ac9 ends here
