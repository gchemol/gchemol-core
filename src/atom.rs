// imports

// [[file:~/Workspace/Programming/gchemol-rs/gchemol-core/gchemol-core.note::*imports][imports:1]]
use guts::prelude::*;
use serde::*;

use crate::element::*;
// imports:1 ends here

// base

// [[file:~/Workspace/Programming/gchemol-rs/gchemol-core/gchemol-core.note::*base][base:1]]
pub type Vector3f = vecfx::Vector3f;

pub(crate) type Point3 = [f64; 3];

/// Atom is the smallest particle still characterizing a chemical element.
///
/// # Reference
///
/// https://goldbook.iupac.org/html/A/A00493.html
///
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Atom {
    /// Chemical element or dummy atom.
    kind: AtomKind,

    /// Atom position.
    position: Vector3f,

    /// Atom label.
    label: Option<String>,

    /// Vector quantity equal to the derivative of the position vector with respect to time
    velocity: Vector3f,

    /// Atomic mass
    mass: Option<f64>,

    /// Atomic momentum vector
    momentum: Vector3f,

    /// Atomic partial charge
    partial_charge: f64,
}

impl Default for Atom {
    fn default() -> Self {
        Self {
            kind: "C".into(),
            position: Vector3f::new(0.0, 0.0, 0.0),
            momentum: Vector3f::new(0.0, 0.0, 0.0),
            velocity: Vector3f::new(0.0, 0.0, 0.0),
            partial_charge: 0.0,

            // FIXME
            mass: None,
            label: None,
        }
    }
}

impl Atom {
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

    /// Vector quantity equal to the product of mass and velocity.
    pub fn momentum(&self) -> Vector3f {
        self.momentum
    }

    /// TODO: momentum, momenta
    pub fn set_momentum<P: Into<Vector3f>>(&mut self, m: P) {
        self.momentum = m.into();
    }

    /// Set atom label
    pub fn set_label<S: Into<String>>(&mut self, lbl: S) {
        self.label = Some(lbl.into());
    }

    /// Return the user defined atom label, if not return the elment symbol.
    pub fn label(&self) -> &str {
        if let Some(ref l) = self.label {
            return l;
        }

        // default atom label: element symbol
        self.symbol()
    }

    /// Set atom symbol.
    pub fn set_symbol<S: Into<AtomKind>>(&mut self, symbol: S) {
        self.kind = symbol.into()
    }
}
// base:1 ends here

// convert

// [[file:~/Workspace/Programming/gchemol-rs/gchemol-core/gchemol-core.note::*convert][convert:1]]
use std::convert::From;
use std::str::FromStr;

impl FromStr for Atom {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self> {
        let parts: Vec<_> = line.split_whitespace().collect();
        if parts.len() != 4 {
            bail!("Incorrect number of data fields: {:?}", line);
        }

        let sym = parts[0];
        let px: f64 = parts[1].parse()?;
        let py: f64 = parts[2].parse()?;
        let pz: f64 = parts[3].parse()?;

        let atom = Atom::new(sym, [px, py, pz]);

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
// convert:1 ends here

// test

// [[file:~/Workspace/Programming/gchemol-rs/gchemol-core/gchemol-core.note::*test][test:1]]
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
}
// test:1 ends here
