// [[file:../gchemol-core.note::*imports][imports:1]]
use crate::{Atom, Molecule};
use crate::{Point3, Vector3f};
// imports:1 ends here

// [[file:../gchemol-core.note::*api/core][api/core:1]]
/// Geometry related methods
impl Molecule {
    /// Translate the whole molecule by a displacement
    pub fn translate<P: Into<Vector3f>>(&mut self, disp: P) {
        let disp: Vector3f = disp.into();
        for &n in self.mapping.right_values() {
            let atom = &mut self.graph[n];
            let p: Vector3f = atom.position().into();
            let position = p + disp;
            atom.set_position(position);
        }
    }

    // FIXME: Running mean algo
    /// Return the center of geometry of molecule (COG).
    pub fn center_of_geometry(&self) -> Point3 {
        let mut p = [0.0; 3];
        for [x, y, z] in self.positions() {
            p[0] += x;
            p[1] += y;
            p[2] += z;
        }

        let n = self.natoms() as f64;
        p[0] /= n;
        p[1] /= n;
        p[2] /= n;

        p
    }

    /// Center the molecule around its center of geometry
    pub fn recenter(&mut self) {
        if self.is_periodic() {
            todo!();
        } else {
            let mut p = self.center_of_geometry();
            for i in 0..3 {
                p[i] *= -1.0;
            }
            self.translate(p);
        }
    }
}
// api/core:1 ends here

// [[file:../gchemol-core.note::95de44db][95de44db]]
use gchemol_geometry::prelude::*;

impl Atom {
    /// Return distance to other atom.
    pub fn distance(&self, other: &Atom) -> f64 {
        use vecfx::*;

        self.position().vecdist(&other.position())
    }
}

impl Molecule {
    /// Return the distance between `atom i` and `atom j`. For periodic
    /// structure, this method will return the distance under the minimum image
    /// convention.
    ///
    /// # Panic
    ///
    /// * if atom indices `i` or `j` out of range.
    #[deprecated(note = "get_distance instead")]
    pub fn distance(&self, i: usize, j: usize) -> f64 {
        self.get_distance(i, j).expect("invalid serial numbers")
    }

    /// Return the distance of two atoms `i`, `j`. For periodic
    /// structure, this method will return the distance under the
    /// minimum image convention. Return None if any serial number is
    /// invalid.
    pub fn get_distance(&self, i: usize, j: usize) -> Option<f64> {
        let pi = self.get_atom(i)?.position();
        let pj = self.get_atom(j)?.position();
        if let Some(lat) = self.lattice {
            lat.distance(pi, pj).into()
        } else {
            pi.distance(pj).into()
        }
    }

    /// Return the angle of three atoms `i`, `j`, `k` in radians,
    /// irrespective periodic images. Return None if any serial number
    /// is invalid.
    pub fn get_angle(&self, i: usize, j: usize, k: usize) -> Option<f64> {
        let pi = self.get_atom(i)?.position();
        let pj = self.get_atom(j)?.position();
        let pk = self.get_atom(k)?.position();
        pi.angle(pj, pk).into()
    }

    /// Return the torsion angle of four atoms `i`, `j`, `k`, `l` in
    /// radians, irrespective periodic images. Return None if any serial
    /// number is invalid.
    pub fn get_torsion(&self, i: usize, j: usize, k: usize, l: usize) -> Option<f64> {
        let pi = self.get_atom(i)?.position();
        let pj = self.get_atom(j)?.position();
        let pk = self.get_atom(k)?.position();
        let pl = self.get_atom(l)?.position();
        pi.torsion(pj, pk, pl).into()
    }
}
// 95de44db ends here
