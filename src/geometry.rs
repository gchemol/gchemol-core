// imports

use vecfx::Vector3f;

use crate::{Atom, Molecule};

// core

type Point3 = [f64; 3];

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

    /// Return the center of mass of molecule (COM).
    pub fn center_of_mass(&self) -> Point3 {
        unimplemented!()
    }

    /// Center the molecule around its center of geometry
    pub fn recenter(&mut self) {
        let mut p = self.center_of_geometry();
        for i in 0..3 {
            p[i] *= -1.0;
        }
        self.translate(p);
    }
}

// distance

impl Atom {
    /// Return distance to other atom.
    pub fn distance(&self, other: &Atom) -> f64 {
        gchemol_geometry::euclidean_distance(self.position(), other.position())
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
    pub fn distance(&self, i: usize, j: usize) -> f64 {
        match (self.get_atom(i), self.get_atom(j)) {
            (Some(ai), Some(aj)) => {
                if let Some(mut lat) = self.lattice {
                    let pi = ai.position();
                    let pj = aj.position();
                    lat.distance(pi, pj)
                } else {
                    ai.distance(aj)
                }
            }
            _ => {
                panic!("atom indices out of range: {}, {}", i, j);
            }
        }
    }
}
