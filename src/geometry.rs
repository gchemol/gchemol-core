// [[file:../gchemol-core.note::8df18d4c][8df18d4c]]
use crate::common::*;
use crate::{Atom, Molecule};
use crate::{Point3, Vector3f};
// 8df18d4c ends here

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

// [[file:../gchemol-core.note::b3195ba1][b3195ba1]]
impl Molecule {
    /// Superimpose structure onto `mol_ref` which will be held fixed
    /// during alignment. Return superposition rmsd on done.
    ///
    /// # NOTE
    /// * The atoms must be in one-to-one mapping with atoms in `mol_ref`
    /// * The structure could be mirrored for better alignment.
    /// * Heavy atoms have more weights.
    pub fn superimpose_onto(&mut self, mol_ref: &Molecule, selection: Option<&[usize]>) -> f64 {
        use gchemol_geometry::Superimpose;

        let (positions_this, positions_prev, weights) = if let Some(selected) = selection {
            let this = selected.iter().map(|&i| self.get_atom(i).unwrap().position()).collect_vec();
            let prev = selected.iter().map(|&i| mol_ref.get_atom(i).unwrap().position()).collect_vec();
            let weights = selected.iter().map(|&i| self.get_atom(i).unwrap().get_mass().unwrap()).collect_vec();
            (this, prev, weights)
        } else {
            (
                self.positions().collect_vec(),
                mol_ref.positions().collect_vec(),
                self.masses().collect_vec(),
            )
        };
        assert_eq!(positions_this.len(), positions_prev.len());
        assert_eq!(positions_this.len(), weights.len());
        let sp1 = Superimpose::new(&positions_this).onto(&positions_prev, weights.as_slice().into());
        let mut positions_this_mirrored = positions_this.clone();
        positions_this_mirrored.mirror_invert();
        let sp2 = Superimpose::new(&positions_this_mirrored).onto(&positions_prev, weights.as_slice().into());
        let positions_this_all = self.positions().collect_vec();
        let (positions_new, rmsd) = if sp1.rmsd < sp2.rmsd {
            (sp1.apply(&positions_this_all), sp1.rmsd)
        } else {
            let mut positions_this_all_mirrored = positions_this_all.clone();
            positions_this_all_mirrored.mirror_invert();
            (sp2.apply(&positions_this_all_mirrored), sp2.rmsd)
        };

        self.set_positions(positions_new);
        rmsd
    }
}
// b3195ba1 ends here
