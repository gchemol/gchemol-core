// [[file:../gchemol-core.note::22d13ff7][22d13ff7]]
use std::collections::HashMap;

use gut::prelude::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::atom::{Point3, Vector3f};
use crate::{Atom, Molecule};
// 22d13ff7 ends here

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

// [[file:../gchemol-core.note::42021a2c][42021a2c]]
use serde_json::{Map, Value};

/// A container storing extra properties for any
/// serializable/deserializable objects managed as json key/value
/// pairs
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct PropertyStore {
    data: Map<String, Value>,
}

impl PropertyStore {
    /// Returns true if the map contains a value for the specified key.
    pub fn contains_key(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    /// Retrieve property associated with the `key`.
    pub fn load<D: DeserializeOwned>(&self, key: &str) -> Result<D> {
        if let Some(value) = self.data.get(key) {
            let d = serde_json::from_value(value.clone()).with_context(|| format!("Failed to deserialize data for key {:?}", key))?;
            Ok(d)
        } else {
            bail!("Failed to get property with key {:?}", key)
        }
    }

    /// Store property associatd with the `key`
    pub fn store<D: Serialize>(&mut self, key: &str, value: D) -> Result<()> {
        let value = serde_json::to_value(&value)?;
        self.data.insert(key.into(), value);
        Ok(())
    }

    /// Discard stored property associated with the `key`.
    pub fn discard(&mut self, key: &str) {
        let _ = self.data.remove(key);
    }
}
// 42021a2c ends here

// [[file:../gchemol-core.note::7113c55b][7113c55b]]
impl PropertyStore {
    /// Take the stored property associated with the `key` out of the
    /// `PropertyStore`, leaving no copy.
    pub fn take<D: DeserializeOwned>(&mut self, key: &str) -> Result<D> {
        if let Some(value) = self.data.remove(key) {
            let d = serde_json::from_value(value).with_context(|| format!("Failed to deserialize data for key {:?}", key))?;
            Ok(d)
        } else {
            bail!("Failed to get property with key {:?}", key)
        }
    }

    /// Clears all stored data.
    #[inline]
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Returns the number of stored items.
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if `PropertyStore` contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Access to the inner map.
    pub fn raw_map(&self) -> &Map<String, Value> {
        &self.data
    }

    /// Mut access to the inner map.
    pub fn raw_map_mut(&mut self) -> &mut Map<String, Value> {
        &mut self.data
    }
}
// 7113c55b ends here

// [[file:../gchemol-core.note::4fe101ef][4fe101ef]]
#[test]
fn test_atom_store() {
    // store builtin types
    let mut x = PropertyStore::default();
    let d = [1, 2, 3];
    x.store("k", d);
    let x: [usize; 3] = x.load("k").unwrap();
    assert_eq!(d, x);

    // Store a custom struct
    #[derive(Debug, Default, Eq, PartialEq, Clone, Serialize, Deserialize)]
    struct A {
        a: usize,
        b: isize,
    }

    let a = A::default();
    let mut x = PropertyStore::default();
    x.store("a", &a);
    let a_loaded: A = x.load("a").unwrap();
    assert_eq!(a_loaded, a);

    let a_took: A = x.take("a").unwrap();
    assert_eq!(x.contains_key("a"), false);
}
// 4fe101ef ends here
