// [[file:../gchemol-core.note::*imports][imports:1]]
use std::collections::HashMap;

use gut::prelude::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json;

use crate::atom::{Point3, Vector3f};
use crate::{Atom, Molecule};
// imports:1 ends here

// [[file:../gchemol-core.note::2484a0c8][2484a0c8]]
#[cfg(feature = "adhoc")]
/// Extra properties for `Atom`.
impl Atom {
    /// Vector quantity equal to the product of mass and velocity.
    pub fn get_momentum(&self) -> Option<Point3> {
        self.mass.map(|m| (m * self.velocity).into())
    }

    /// Set atom velocity.
    pub fn set_velocity<P: Into<Vector3f>>(&mut self, m: P) {
        self.velocity = m.into();
    }

    /// Return atom velocity
    pub fn velocity(&self) -> Point3 {
        self.velocity.into()
    }
}

#[cfg(feature = "adhoc")]
/// Extra properties for `Molecule`.
impl Molecule {
    /// Set atom `sn` 's velocity as `m`
    ///
    /// # Panics
    ///
    /// * panic if there is no atom associated with `sn`.
    pub fn set_velocity<P: Into<Vector3f>>(&mut self, sn: usize, m: P) {
        if let Some(atom) = self.get_atom_mut(sn) {
            atom.set_velocity(m);
        } else {
            panic!("invalid sn: {}", sn);
        }
    }

    /// Set velocities of atoms in sequential order.
    pub fn set_velocities<T, M>(&mut self, velocities: T)
    where
        T: IntoIterator<Item = M>,
        M: Into<Vector3f>,
    {
        for (sn, m) in self.serial_numbers().zip(velocities.into_iter()) {
            let atom = self.get_atom_mut(sn).unwrap();
            atom.set_velocity(m);
        }
    }

    /// Return an iterator over atom velocities.
    pub fn velocities(&self) -> impl Iterator<Item = Point3> + '_ {
        self.atoms().map(move |(_, atom)| atom.velocity())
    }
}
// 2484a0c8 ends here

// [[file:../gchemol-core.note::*adhoc properties][adhoc properties:1]]
/// A container storing extra information managed as key/value pairs
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct PropertyStore {
    data: HashMap<String, String>,
}

impl PropertyStore {
    /// Returns true if the map contains a value for the specified key.
    pub fn contains_key(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    /// Retrieve property associated with the `key`
    pub fn load<D: DeserializeOwned>(&self, key: &str) -> Result<D> {
        if let Some(serialized) = self.data.get(key) {
            let d = serde_json::from_str(&serialized)
                .with_context(|| format!("Failed to deserialize data for key {:?}", key))?;
            Ok(d)
        } else {
            bail!("Failed to get property with key {:?}", key)
        }
    }

    /// Store property associatd with the `key`
    pub fn store<D: Serialize>(&mut self, key: &str, value: D) {
        let serialized = serde_json::to_string(&value).unwrap();
        self.data.insert(key.into(), serialized);
    }

    /// Discard property associated with the `key`
    pub fn discard(&mut self, key: &str) {
        self.data.remove(key);
    }
}
// adhoc properties:1 ends here

// [[file:../gchemol-core.note::*test][test:1]]
#[test]
fn test_atom_store() {
    let mut x = PropertyStore::default();
    let d = [1, 2, 3];
    x.store("k", d);
    let x: [usize; 3] = x.load("k").unwrap();
    assert_eq!(d, x);
}
// test:1 ends here
