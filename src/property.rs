// imports

// [[file:~/Workspace/Programming/gchemol-rs/gchemol-core/gchemol-core.note::*imports][imports:1]]
use std::collections::HashMap;

use guts::prelude::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json;
// imports:1 ends here

// impl

// [[file:~/Workspace/Programming/gchemol-rs/gchemol-core/gchemol-core.note::*impl][impl:1]]
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
// impl:1 ends here

// test

// [[file:~/Workspace/Programming/gchemol-rs/gchemol-core/gchemol-core.note::*test][test:1]]
#[test]
fn test_atom_store() {
    let mut x = PropertyStore::default();
    let d = [1, 2, 3];
    x.store("k", d);
    let x: [usize; 3] = x.load("k").unwrap();
    assert_eq!(d, x);
}
// test:1 ends here
