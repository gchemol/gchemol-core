// [[file:../gchemol-core.note::*imports][imports:1]]
use crate::molecule::Molecule;
use crate::Point3;
// imports:1 ends here

// [[file:../gchemol-core.note::*core][core:1]]
type Matrix3 = [[f64; 3]; 3];

/// Geometry related methods
impl Molecule {
    /// Return molecule's inertia matrix (3x3) in reference to molecule's center of
    /// mass
    pub fn inertia_matrix(&self) -> Matrix3 {
        // calculate the center of mass of the rigid molecule
        let com = self.center_of_mass();

        // set origin at the center of mass of the molecule.
        let positions = self.positions().map(|p| [p[0] - com[0], p[1] - com[1], p[2] - com[2]]);

        // calculate the inertia matrix
        let mut inertia_matrix = Matrix3::default();
        for ([x, y, z], m) in positions.zip(self.masses()) {
            inertia_matrix[0][0] += m * (y.powi(2) + z.powi(2));
            inertia_matrix[1][1] += m * (x.powi(2) + z.powi(2));
            inertia_matrix[2][2] += m * (x.powi(2) + y.powi(2));
            inertia_matrix[0][1] -= m * x * y;
            inertia_matrix[0][2] -= m * x * z;
            inertia_matrix[1][2] -= m * y * z;
        }

        // make inertia matrix symmetric
        inertia_matrix[1][0] = inertia_matrix[0][1];
        inertia_matrix[2][0] = inertia_matrix[0][2];
        inertia_matrix[2][1] = inertia_matrix[1][2];

        inertia_matrix
    }

    /// Return the center of mass of molecule (COM).
    pub fn center_of_mass(&self) -> Point3 {
        use vecfx::*;

        let masses: Vec<_> = self.masses().collect();
        let mut p = [0.0; 3];
        for ([x, y, z], m) in self.positions().zip(masses.iter()) {
            p[0] += x * m;
            p[1] += y * m;
            p[2] += z * m;
        }

        let s = masses.sum();
        assert!(s.is_sign_positive(), "invalid masses: {:?}", masses);
        p[0] /= s;
        p[1] /= s;
        p[2] /= s;

        p
    }
}
// core:1 ends here
