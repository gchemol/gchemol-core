// imports

use vecfx::Vector3f;

use crate::Molecule;

// core

type Point3 = [f64; 3];

#[cfg(feature="adhoc")]
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
