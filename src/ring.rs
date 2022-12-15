// [[file:../gchemol-core.note::77290756][77290756]]
//! Find rings in a `Molecule`
//!
//! Credit:
//!
//! Heavily inspired by the codes developed by vitroid: https://github.com/vitroid/CountRings
// 77290756 ends here

// [[file:../gchemol-core.note::0221ebf7][0221ebf7]]
use crate::common::*;
use crate::molecule::Molecule;

use std::collections::HashSet;

pub type Rings = Vec<HashSet<usize>>;
// 0221ebf7 ends here

// [[file:../gchemol-core.note::ca262281][ca262281]]
fn find_ring(
    mol: &Molecule,    // parent molecule
    members: &[usize], // current node list
    max: usize,        // max ring size?
) -> (usize, Rings) {
    let n = members.len();
    if n > max {
        return (max, vec![]);
    }

    let mut results = vec![];
    let last = members[n - 1];
    let mut max = max;
    for adj in mol.connected(last) {
        if members.contains(&adj) {
            if adj == members[0] {
                // Ring is closed.
                // It is the best and unique answer.
                let s: HashSet<_> = members.to_vec().into_iter().collect();
                if !shortcuts(mol, members) {
                    return (members.len(), vec![s]);
                }
            } else {
                // Shortcut ring
            }
        } else {
            let mut ms = members.to_vec();
            ms.push(adj);
            let (newmax, newres) = find_ring(mol, &ms, max);
            if newmax < max {
                max = newmax;
                results = newres;
            } else if newmax == max {
                results.extend(newres);
            }
        }
    }

    (max, results)
}

fn shortcuts(mol: &Molecule, members: &[usize]) -> bool {
    let n = members.len();
    for i in 0..n {
        for j in (i + 1)..n {
            let d = (j - i).min(n - (j - i));
            if d > shortest_pathlen(mol, members[i], members[j]) {
                return true;
            }
        }
    }
    return false;
}

// FIXME: caching
fn shortest_pathlen(mol: &Molecule, i: usize, j: usize) -> usize {
    if let Some(n) = mol.nbonds_between(i, j) {
        n
    } else {
        0
    }
}

fn find_rings(mol: &Molecule, max_ring_size: usize) -> Rings {
    let mut rings = vec![];
    for x in mol.numbers() {
        let mut neis = mol.connected(x).collect_vec();
        neis.sort();
        for p in neis.iter().combinations(2) {
            let y = p[0];
            let z = p[1];
            let triplet = [*y, x, *z];
            let (max, mut results) = find_ring(mol, &triplet, max_ring_size);
            for i in results {
                if !rings.contains(&i) {
                    rings.push(i);
                }
            }
        }
    }

    rings
}
// ca262281 ends here

// [[file:../gchemol-core.note::92cea8ed][92cea8ed]]
impl Molecule {
    /// Find rings up to `nmax` atoms in `Molecule`.
    ///
    /// # Arguments
    /// * nmax: max ring size to be searched.
    pub fn find_rings(&self, nmax: usize) -> Rings {
        let rings = find_rings(self, nmax);

        rings
    }
}
// 92cea8ed ends here
