// imports

// [[file:~/Workspace/Programming/gchemol-rs/gchemol-core/gchemol-core.note::*imports][imports:1]]
use serde::*;

use crate::property::PropertyStore;
// imports:1 ends here

// base

// [[file:~/Workspace/Programming/gchemol-rs/gchemol-core/gchemol-core.note::*base][base:1]]
/// https://en.wikipedia.org/wiki/Bond_order
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Deserialize, Serialize)]
pub enum BondKind {
    Dummy,
    Partial,
    Single,
    Aromatic,
    Double,
    Triple,
    Quadruple,
}

/// There is a chemical bond between two atoms or groups of atoms in the case
/// that the forces acting between them are such as to lead to the formation of
/// an aggregate with sufficient stability to make it convenient for the chemist
/// to consider it as an independent 'molecular species'.
///
/// # Reference
/// https://goldbook.iupac.org/html/B/B00697.html
///
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Bond {
    /// Arbitrary property stored in key-value pair. Key is a string type, but
    /// it is the responsibility of the user to correctly interpret the value.
    pub properties: PropertyStore,

    /// bond type
    kind: BondKind,

    /// bond label
    label: String,

    /// set this attribute for arbitrary bond order
    order: Option<f64>,
}

impl Default for Bond {
    fn default() -> Self {
        Bond {
            kind: BondKind::Single,
            label: String::new(),
            order: None,
            properties: PropertyStore::default(),
        }
    }
}

impl Bond {
    /// Returns bond kind/type.
    pub fn kind(&self) -> BondKind {
        self.kind
    }

    /// Change bond kind/type.
    pub fn set_kind(&mut self, k: BondKind) {
        self.kind = k;
    }

    /// Change current bond order to `o`.
    pub fn set_order(&mut self, o: f64) {
        debug_assert!(o >= 0.0);
        self.order = Some(o);
    }

    /// Change bond label.
    pub fn set_label<T: Into<String>>(&mut self, l: T) {
        self.label = l.into();
    }

    /// Returns bond label.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Return true if `Bond` is a dummy bond.
    pub fn is_dummy(&self) -> bool {
        self.kind == BondKind::Dummy
    }

    /// Return true if `Bond` is a single bond.
    pub fn is_single(&self) -> bool {
        self.kind == BondKind::Single
    }

    /// Return true if `Bond` is a double bond.
    pub fn is_double(&self) -> bool {
        self.kind == BondKind::Double
    }

    /// Return bond order
    pub fn order(&self) -> f64 {
        if let Some(order) = self.order {
            order
        } else {
            match self.kind {
                BondKind::Dummy => 0.0,
                BondKind::Partial => 0.5,
                BondKind::Single => 1.0,
                BondKind::Aromatic => 1.5,
                BondKind::Double => 2.0,
                BondKind::Triple => 3.0,
                BondKind::Quadruple => 4.0,
            }
        }
    }

    /// Create a single bond
    pub fn single() -> Self {
        Bond {
            kind: BondKind::Single,
            ..Default::default()
        }
    }

    /// Create a double bond
    pub fn double() -> Self {
        Bond {
            kind: BondKind::Double,
            ..Default::default()
        }
    }

    /// Create a triple bond
    pub fn triple() -> Self {
        Bond {
            kind: BondKind::Triple,
            ..Default::default()
        }
    }

    /// Create an aromatic bond
    pub fn aromatic() -> Self {
        Bond {
            kind: BondKind::Aromatic,
            ..Default::default()
        }
    }

    /// Create a weak bond
    pub fn partial() -> Self {
        Bond {
            kind: BondKind::Partial,
            ..Default::default()
        }
    }

    /// Create a quadruple bond
    pub fn quadruple() -> Self {
        Bond {
            kind: BondKind::Quadruple,
            ..Default::default()
        }
    }

    /// Create a dummy bond
    pub fn dummy() -> Self {
        Bond {
            kind: BondKind::Dummy,
            ..Default::default()
        }
    }
}
// base:1 ends here

// test

// [[file:~/Workspace/Programming/gchemol-rs/gchemol-core/gchemol-core.note::*test][test:1]]
#[test]
fn test_bond() {
    let b = Bond::default();
    assert_eq!(1.0, b.order());
    assert_eq!(1.0, Bond::single().order());
    assert_eq!(2.0, Bond::double().order());
    assert_eq!(3.0, Bond::triple().order());
}
// test:1 ends here
