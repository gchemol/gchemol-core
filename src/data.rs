// imports

//===============================================================================#
//   DESCRIPTION: provide basic chemical data for atoms and molecules
//
//       OPTIONS:  ---
//  REQUIREMENTS:  ---
//         NOTES:  ---
//        AUTHOR:  Wenping Guo <ybyygu@gmail.com>
//       LICENCE:  GPL version 3
//       CREATED:  <2018-04-12 Thu 14:40>
//       UPDATED:  <2020-02-24 Mon 16:33>
//===============================================================================#

use crate::{Atom, AtomKind, Molecule};

// radii
// Element radii data taking from: https://mendeleev.readthedocs.io/en/stable/data.html

// Data in columns:
// covalent_radii_single covalent_radii_double, covalent_radii_triple, vdw_radii


const RADII_DATA: [[f64; 4]; 118] = [
    [0.32, 0.32, 0.32, 1.1],
    [0.46, 0.46, 0.46, 1.4],
    [1.33, 1.24, 1.24, 1.82],
    [1.02, 0.9, 0.85, 1.53],
    [0.85, 0.78, 0.73, 1.92],
    [0.75, 0.67, 0.6, 1.7],
    [0.71, 0.6, 0.54, 1.55],
    [0.63, 0.57, 0.53, 1.52],
    [0.64, 0.59, 0.53, 1.47],
    [0.67, 0.96, 0.96, 1.54],
    [1.55, 1.6, 1.6, 2.27],
    [1.39, 1.32, 1.27, 1.73],
    [1.26, 1.13, 1.11, 1.84],
    [1.16, 1.07, 1.02, 2.1],
    [1.11, 1.02, 0.94, 1.8],
    [1.03, 0.94, 0.95, 1.8],
    [0.99, 0.95, 0.93, 1.75],
    [0.96, 1.07, 0.96, 1.88],
    [1.96, 1.93, 1.93, 2.75],
    [1.71, 1.47, 1.33, 2.31],
    [1.48, 1.16, 1.14, 2.15],
    [1.36, 1.17, 1.08, 2.11],
    [1.34, 1.12, 1.06, 2.07],
    [1.22, 1.11, 1.03, 2.06],
    [1.19, 1.05, 1.03, 2.05],
    [1.16, 1.09, 1.02, 2.04],
    [1.11, 1.03, 0.96, 2.0],
    [1.1, 1.01, 1.01, 1.97],
    [1.12, 1.15, 1.2, 1.96],
    [1.18, 1.2, 1.2, 2.01],
    [1.24, 1.17, 1.21, 1.87],
    [1.21, 1.11, 1.14, 2.11],
    [1.21, 1.14, 1.06, 1.85],
    [1.16, 1.07, 1.07, 1.9],
    [1.14, 1.09, 1.1, 1.85],
    [1.17, 1.21, 1.08, 2.02],
    [2.1, 2.02, 2.02, 3.03],
    [1.85, 1.57, 1.39, 2.49],
    [1.63, 1.3, 1.24, 2.32],
    [1.54, 1.27, 1.21, 2.23],
    [1.47, 1.25, 1.16, 2.18],
    [1.38, 1.21, 1.13, 2.17],
    [1.28, 1.2, 1.1, 2.16],
    [1.25, 1.14, 1.03, 2.13],
    [1.25, 1.1, 1.06, 2.1],
    [1.2, 1.17, 1.12, 2.1],
    [1.28, 1.39, 1.37, 2.11],
    [1.36, 1.44, 1.44, 2.18],
    [1.42, 1.36, 1.46, 1.93],
    [1.4, 1.3, 1.32, 2.17],
    [1.4, 1.33, 1.27, 2.06],
    [1.36, 1.28, 1.21, 2.06],
    [1.33, 1.29, 1.25, 1.98],
    [1.31, 1.35, 1.22, 2.16],
    [2.32, 2.09, 2.09, 3.43],
    [1.96, 1.61, 1.49, 2.68],
    [1.8, 1.39, 1.39, 2.43],
    [1.63, 1.37, 1.31, 2.42],
    [1.76, 1.38, 1.28, 2.4],
    [1.74, 1.37, 1.37, 2.39],
    [1.73, 1.35, 1.35, 2.38],
    [1.72, 1.34, 1.34, 2.36],
    [1.68, 1.34, 1.34, 2.35],
    [1.69, 1.35, 1.32, 2.34],
    [1.68, 1.35, 1.35, 2.33],
    [1.67, 1.33, 1.33, 2.31],
    [1.66, 1.33, 1.33, 2.3],
    [1.65, 1.33, 1.33, 2.29],
    [1.64, 1.31, 1.31, 2.27],
    [1.7, 1.29, 1.29, 2.26],
    [1.62, 1.31, 1.31, 2.24],
    [1.52, 1.28, 1.22, 2.23],
    [1.46, 1.26, 1.19, 2.22],
    [1.37, 1.2, 1.15, 2.18],
    [1.31, 1.19, 1.1, 2.16],
    [1.29, 1.16, 1.09, 2.16],
    [1.22, 1.15, 1.07, 2.13],
    [1.23, 1.12, 1.1, 2.13],
    [1.24, 1.21, 1.23, 2.14],
    [1.33, 1.42, 1.42, 2.23],
    [1.44, 1.42, 1.5, 1.96],
    [1.44, 1.35, 1.37, 2.02],
    [1.51, 1.41, 1.35, 2.07],
    [1.45, 1.35, 1.29, 1.97],
    [1.47, 1.38, 1.38, 2.02],
    [1.42, 1.45, 1.33, 2.2],
    [2.23, 2.18, 2.18, 3.48],
    [2.01, 1.73, 1.59, 2.83],
    [1.86, 1.53, 1.4, 2.47],
    [1.75, 1.43, 1.36, 2.45],
    [1.69, 1.38, 1.29, 2.43],
    [1.7, 1.34, 1.18, 2.41],
    [1.71, 1.36, 1.16, 2.39],
    [1.72, 1.35, 1.35, 2.43],
    [1.66, 1.35, 1.35, 2.44],
    [1.66, 1.36, 1.36, 2.45],
    [1.68, 1.39, 1.39, 2.44],
    [1.68, 1.4, 1.4, 2.45],
    [1.65, 1.4, 1.4, 2.45],
    [1.67, 1.67, 1.67, 2.45],
    [1.73, 1.39, 1.39, 2.46],
    [1.76, 1.76, 1.76, 2.46],
    [1.61, 1.41, 1.41, 2.46],
    [1.57, 1.4, 1.31, 2.46],
    [1.49, 1.36, 1.26, 2.46],
    [1.43, 1.28, 1.21, 2.46],
    [1.41, 1.28, 1.19, 2.46],
    [1.34, 1.25, 1.18, 2.46],
    [1.29, 1.25, 1.13, 2.46],
    [1.28, 1.16, 1.18, 2.46],
    [1.21, 1.16, 1.18, 2.46],
    [1.22, 1.37, 1.3, 2.46],
    [1.36, 1.36, 1.36, 2.46],
    [1.43, 1.43, 1.43, 2.46],
    [1.62, 1.62, 1.62, 2.46],
    [1.75, 1.75, 1.75, 2.46],
    [1.65, 1.65, 1.65, 2.46],
    [1.57, 1.57, 1.57, 2.46],
];

/// Return covalent radius for single, double, or triple bonds
fn get_cov_radius(element_number: usize, bond_order: usize) -> Option<f64> {
    if element_number > 0 && element_number <= RADII_DATA.len() {
        // only for single, double, or triple bond
        if bond_order > 0 && bond_order <= 3 {
            let irow = element_number - 1;
            let icol = bond_order - 1;

            let r = RADII_DATA[irow][icol];
            return Some(r);
        }
    }

    None
}

/// Return Van der Waals radius if any
fn get_vdw_radius(element_number: usize) -> Option<f64> {
    // column index to vdw radii
    let icol = 3;
    if element_number > 0 && element_number <= RADII_DATA.len() {
        let irow = element_number - 1;
        let r = RADII_DATA[irow][icol];
        return Some(r);
    }
    None
}

// masses
// # get from python mendeleev package

const MASSES_DATA: [f64; 118] = [
    1.008000, 4.002602, 6.940000, 9.012183, 10.810000, 12.011000, 14.007000, 15.999000, 18.998403, 20.179700,
    22.989769, 24.305000, 26.981538, 28.085000, 30.973762, 32.060000, 35.450000, 39.948000, 39.098300, 40.078000,
    44.955908, 47.867000, 50.941500, 51.996100, 54.938044, 55.845000, 58.933194, 58.693400, 63.546000, 65.380000,
    69.723000, 72.630000, 74.921595, 78.971000, 79.904000, 83.798000, 85.467800, 87.620000, 88.905840, 91.224000,
    92.906370, 95.950000, 97.907210, 101.070000, 102.905500, 106.420000, 107.868200, 112.414000, 114.818000,
    118.710000, 121.760000, 127.600000, 126.904470, 131.293000, 132.905452, 137.327000, 138.905470, 140.116000,
    140.907660, 144.242000, 144.912760, 150.360000, 151.964000, 157.250000, 158.925350, 162.500000, 164.930330,
    167.259000, 168.934220, 173.045000, 174.966800, 178.490000, 180.947880, 183.840000, 186.207000, 190.230000,
    192.217000, 195.084000, 196.966569, 200.592000, 204.380000, 207.200000, 208.980400, 209.000000, 210.000000,
    222.000000, 223.000000, 226.000000, 227.000000, 232.037700, 231.035880, 238.028910, 237.000000, 244.000000,
    243.000000, 247.000000, 247.000000, 251.000000, 252.000000, 257.000000, 258.000000, 259.000000, 262.000000,
    267.000000, 268.000000, 271.000000, 274.000000, 269.000000, 276.000000, 281.000000, 281.000000, 285.000000,
    286.000000, 289.000000, 288.000000, 293.000000, 294.000000, 294.000000,
];

fn get_atom_mass(atom: &Atom) -> Option<f64> {
    match atom.kind() {
        AtomKind::Element(n) => {
            assert!(*n > 0, "invalid element number");
            Some(MASSES_DATA[n - 1])
        }
        AtomKind::Dummy(_) => None,
    }
}

// atom

/// Core data for `Atom`
impl Atom {
    /// Access covalent radius of atom. Return None if no data available or atom is dummy.
    pub fn get_cov_radius(&self) -> Option<f64> {
        get_cov_radius(self.number(), 1)
    }

    /// Access Van der Waals radius of atom.
    /// Return None if no data available
    pub fn get_vdw_radius(&self) -> Option<f64> {
        get_vdw_radius(self.number())
    }

    #[deprecated(note = "use get_cov_radius instead")]
    /// Access covalent radius of atom. Return None if no data available or atom is dummy.
    pub fn cov_radius(&self) -> Option<f64> {
        self.get_cov_radius()
    }

    /// Get mass in atomic mass unit. Return None if atom is dummy.
    pub fn get_mass(&self) -> Option<f64> {
        self.mass.or(get_atom_mass(&self))
    }

    #[deprecated(note = "Use get_vdw_radius instead")]
    /// Access Van der Waals radius of atom.
    /// Return None if no data available
    pub fn vdw_radius(&self) -> Option<f64> {
        self.get_vdw_radius()
    }
}

// molecule

impl Molecule {
    /// Returns `Molecule` created from the internal database (mainly for tests).
    pub fn from_database(name: &str) -> Self {
        let ch4 = "
  C  -0.0000   -0.0000    0.0000
  H   1.0900   -0.0000    0.0000
  H  -0.3633    1.0277    0.0000
  H  -0.3633   -0.5138    0.8900
  H  -0.3633   -0.5138   -0.8900 ";

        let h2o = "
O    -1.4689     2.1375     0.0000
H    -0.5089     2.1375     0.0000
H    -1.7894     3.0424     0.0000
";

        let hcn = "
H		-2.5671751	1.2900188	0.0000000
C		-1.4971751	1.2900188	0.0000000
N		-0.3505751	1.2900188	0.0000000
";

        match name {
            "HCN" => {
                let atoms = hcn.trim().lines().filter_map(|line| line.parse::<Atom>().ok());
                Molecule::from_atoms(atoms)
            }
            "H2O" => {
                let atoms = h2o.trim().lines().filter_map(|line| line.parse::<Atom>().ok());
                Molecule::from_atoms(atoms)
            }
            "CH4" => {
                let atoms = ch4.trim().lines().filter_map(|line| line.parse::<Atom>().ok());
                Molecule::from_atoms(atoms)
            }
            _ => unimplemented!(),
        }
    }
}

// test

#[test]
fn test_atom_data() {
    // carbon atom
    let atom1 = Atom::new("C", [-0.90203687, 0.62555259, 0.0081889]);
    // dummy atom
    let atom2 = Atom::new("X", [-0.54538244, -0.38325741, 0.0081889]);

    assert!(atom1.get_cov_radius().is_some());
    assert!(atom1.get_vdw_radius().is_some());
    assert!(atom2.get_cov_radius().is_none());

    // atom mass
    assert_eq!(atom1.get_mass(), Some(12.011));
    assert_eq!(atom2.get_mass(), None);
}
