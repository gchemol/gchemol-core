// [[file:../gchemol-core.note::*imports][imports:1]]
use gut::prelude::*;

use crate::element::*;
use crate::property::PropertyStore;
// imports:1 ends here

// [[file:../gchemol-core.note::d14b8035][d14b8035]]
/// nalgebra 3D Vector
pub type Vector3f = vecfx::Vector3f;

/// Plain 3D array
pub type Point3 = [f64; 3];

/// Atom is the smallest particle still characterizing a chemical element.
///
/// # Reference
///
/// https://goldbook.iupac.org/html/A/A00493.html
///
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Atom {
    /// Arbitrary property stored in key-value pair. Key is a string type, but
    /// it is the responsibility of the user to correctly interpret the value.
    pub properties: PropertyStore,

    /// Chemical element or dummy atom.
    kind: AtomKind,

    /// Atom position.
    position: Vector3f,

    /// Atom label.
    label: Option<String>,

    /// Vector quantity equal to the derivative of the position vector with respect to time
    pub(crate) velocity: Vector3f,

    /// Atomic mass
    pub(crate) mass: Option<f64>,

    /// Atomic partial charge
    pub(crate) partial_charge: Option<f64>,

    /// Indicates freezing atom
    freezing: [bool; 3],
}

impl Default for Atom {
    fn default() -> Self {
        Self {
            properties: PropertyStore::default(),
            kind: "C".into(),
            position: Vector3f::new(0.0, 0.0, 0.0),
            velocity: Vector3f::new(0.0, 0.0, 0.0),
            partial_charge: None,

            // FIXME: not so sure these fields are necessary
            mass: None,
            label: None,
            freezing: [false; 3],
        }
    }
}

impl Atom {
    /// Construct `Atom` from element symbol and Cartesian coordinates.
    pub fn new<S: Into<AtomKind>, P: Into<Vector3f>>(s: S, p: P) -> Self {
        Self {
            kind: s.into(),
            position: p.into(),
            ..Default::default()
        }
    }

    /// Return element symbol
    pub fn symbol(&self) -> &str {
        self.kind.symbol()
    }

    /// Return atomic number
    pub fn number(&self) -> usize {
        self.kind.number()
    }

    /// Return atom position in 3D Cartesian coordinates
    pub fn position(&self) -> Point3 {
        self.position.into()
    }

    /// Set atom position in 3D Cartesian coordinates
    pub fn set_position<P: Into<Vector3f>>(&mut self, p: P) {
        self.position = p.into();
    }

    /// Return atom kind.
    pub fn kind(&self) -> &AtomKind {
        &self.kind
    }

    /// Set atom label
    pub fn set_label<S: Into<String>>(&mut self, lbl: S) {
        self.label = Some(lbl.into());
    }

    /// Return the user defined atom label, if not return the elment symbol.
    #[deprecated(note = "use get_label instead")]
    pub fn label(&self) -> &str {
        if let Some(ref l) = self.label {
            return l;
        }

        // default atom label: element symbol
        self.symbol()
    }

    /// Return the user defined atom label if defined.
    pub fn get_label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Set atom symbol.
    pub fn set_symbol<S: Into<AtomKind>>(&mut self, symbol: S) {
        self.kind = symbol.into()
    }

    /// Assign atom partial charge, usually for molecular mechanical
    /// calculations.
    pub fn set_partial_charge(&mut self, c: f64) {
        self.partial_charge = Some(c);
    }

    /// Return true if atom is dummy.
    pub fn is_dummy(&self) -> bool {
        match self.kind {
            AtomKind::Element(_) => false,
            AtomKind::Dummy(_) => true,
        }
    }

    /// Return true if atom is an element.
    pub fn is_element(&self) -> bool {
        !self.is_dummy()
    }

    /// Return true if atom has freezing masks in all x, y, z coords
    pub fn is_fixed(&self) -> bool {
        self.freezing.iter().all(|f| *f)
    }

    /// Set freezing mask array for Cartesian coordinates
    pub fn set_freezing(&mut self, freezing: [bool; 3]) {
        self.freezing = freezing;
    }

    /// Return freezing mask array for Cartesian coordinates
    pub fn freezing(&self) -> [bool; 3] {
        self.freezing
    }

    /// Update Cartesian coordinates partially, without changing its freezing
    /// coordinate components (xyz).
    pub fn update_position<P: Into<Vector3f>>(&mut self, p: P) {
        // refuse to update position of freezing atom
        let new_position: Vector3f = p.into();
        for (i, masked) in self.freezing.iter().enumerate() {
            if !*masked {
                self.position[i] = new_position[i];
            }
        }
    }
}
// d14b8035 ends here

// [[file:../gchemol-core.note::fa2e494e][fa2e494e]]
use std::convert::From;
use std::str::FromStr;

impl FromStr for Atom {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self> {
        let parts: Vec<_> = line.split_whitespace().collect();
        let nparts = parts.len();
        ensure!(nparts >= 4, "Incorrect number of data fields: {line:?}");
        let sym = parts[0];
        let px: f64 = parts[1].parse()?;
        let py: f64 = parts[2].parse()?;
        let pz: f64 = parts[3].parse()?;

        let mut atom = Atom::new(sym, [px, py, pz]);
        // HACK: parse velocities
        if nparts >= 6 {
            let vxyz: Vec<_> = parts[4..7].iter().filter_map(|x| x.parse().ok()).collect();
            if vxyz.len() == 3 {
                atom.set_velocity([vxyz[0], vxyz[1], vxyz[2]]);
            }
        }

        Ok(atom)
    }
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:6} {:-12.6} {:-12.6} {:-12.6}",
            self.symbol(),
            self.position[0],
            self.position[1],
            self.position[2]
        )
    }
}

impl<S, P> From<(S, P)> for Atom
where
    S: Into<AtomKind>,
    P: Into<Vector3f>,
{
    fn from(item: (S, P)) -> Self {
        Self::new(item.0, item.1)
    }
}
// fa2e494e ends here

// [[file:../gchemol-core.note::a09f666f][a09f666f]]
#[test]
fn test_atom_basic() {
    let _ = Atom::default();
    let atom = Atom::new("Fe", [9.3; 3]);
    assert_eq!(9.3, atom.position()[0]);
    assert_eq!("Fe", atom.symbol());
    assert_eq!(26, atom.number());

    let mut atom = Atom::new(6, [1.0, 0.0, 0.0]);
    assert_eq!(atom.symbol(), "C");
    atom.set_symbol("H");
    assert_eq!(atom.symbol(), "H");

    let atom = Atom::new("X", [9.3; 3]);
    assert_eq!("X", atom.symbol());
    assert_eq!(0, atom.number());
}

#[test]
fn test_atom_convert() {
    let line = "H 1.0 1.0 1.0";
    let a: Atom = line.parse().unwrap();
    let line = a.to_string();
    let b: Atom = line.parse().unwrap();
    assert_eq!(a.symbol(), b.symbol());
    assert_eq!(a.position(), b.position());

    // from and into
    let a: Atom = (1, [0.0; 3]).into();
    assert_eq!(a.number(), 1);

    // velocity
    let line = "H 1.0 1.0 1.0 2.0 0.0 0";
    let a: Atom = line.parse().unwrap();
    assert_eq!(a.velocity.x, 2.0);
    assert_eq!(a.velocity.y, 0.0);
    assert_eq!(a.velocity.z, 0.0);
}
// a09f666f ends here
