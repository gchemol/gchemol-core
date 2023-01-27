//! Atom orders in Molecule.

// [[file:../gchemol-core.note::*imports][imports:1]]
use crate::common::*;
use crate::Molecule;
// imports:1 ends here

// [[file:../gchemol-core.note::*core][core:1]]
impl Molecule {
    /// Node indices of internal graph object, ordered in serial numbers.
    pub(crate) fn node_indices(&self) -> impl Iterator<Item = NodeIndex> + '_ {
        self.serial_numbers()
            .filter_map(move |sn| self.get_node_index(sn))
            .copied()
    }
}
// core:1 ends here

// [[file:../gchemol-core.note::31e7954d][31e7954d]]
/// Display order of `Atom` in `Molecule`
impl Molecule {
    /// Renumber atoms consecutively from 1.
    pub fn renumber(&mut self) {
        let n = self.natoms();
        let atoms = (1..n + 1).collect_vec();
        self.renumber_using(&atoms);
    }

    /// Renumber atoms by user provided numbers for each atom.
    ///
    /// # NOTE
    /// - number in `atoms` must be unique and in one-to-one mapping to
    ///   original numbering.
    pub fn renumber_using(&mut self, atoms: &[usize]) {
        let nodes: Vec<_> = self.node_indices().collect();
        assert_eq!(nodes.len(), atoms.len());
        self.mapping.clear();

        for (&i, n) in atoms.iter().zip(nodes) {
            self.mapping.insert_no_overwrite(i, n).expect("renumber failure");
        }
    }

    /// Swap the display order of two Atoms `sn1` and `sn2`.
    ///
    /// # Panic
    ///
    /// * Panics if serial numbers `sn1` or `sn2` out of bounds.
    pub fn swap_order(&mut self, sn1: usize, sn2: usize) {
        let n1 = self.remove_atom_sn(sn1).expect("invalid sn1");
        let n2 = self.remove_atom_sn(sn2).expect("invalid sn2");
        self.mapping.insert(sn1, n2);
        self.mapping.insert(sn2, n1);
    }

    /// Reorder the atoms according to the ordering of keys. Keys define 1-to-1
    /// mapping of atoms.
    ///
    /// # Note
    ///
    /// * This method will cause serial numbers renumbered from 1.
    ///
    /// # Panic
    ///
    /// * panics if the size of `keys` is different than the number of atoms.
    pub fn reorder<O>(&mut self, keys: &[O])
    where
        O: std::cmp::Ord,
    {
        assert_eq!(self.natoms(), keys.len(), "keys length is invalid");

        let mut nodes: Vec<_> = self
            .node_indices()
            .enumerate()
            .sorted_by_key(|(i, _)| &keys[*i])
            .map(|(_, n)| n)
            .collect();

        self.mapping.clear();
        for (i, n) in (1..).zip(nodes) {
            self.mapping.insert_no_overwrite(i, n).expect("reorder failure");
        }
    }
}
// 31e7954d ends here

// [[file:../gchemol-core.note::*test][test:1]]
#[test]
fn test_atom_orders() {
    use crate::{Atom, Molecule};

    let mut mol = Molecule::from_database("HCN");
    let a = mol.remove_atom(1).unwrap();
    // serial number: 1 => 4
    mol.add_atom(4, a);

    let d: Vec<_> = mol.atoms().map(|(i, a)| (i, a.symbol())).collect();
    let expected = vec![(2, "C"), (3, "N"), (4, "H")];
    assert_eq!(d, expected);

    mol.renumber();
    let d: Vec<_> = mol.atoms().map(|(i, a)| (i, a.symbol())).collect();
    let expected = vec![(1, "C"), (2, "N"), (3, "H")];
    assert_eq!(d, expected);

    mol.swap_order(1, 3);
    let d: Vec<_> = mol.atoms().map(|(i, a)| (i, a.symbol())).collect();
    let expected = vec![(1, "H"), (2, "N"), (3, "C")];
    assert_eq!(d, expected);

    // sort by atomic numbers, hydrogen first
    let numbers: Vec<_> = mol.atomic_numbers().collect();
    mol.reorder(&numbers);
    let d: Vec<_> = mol.atoms().map(|(i, a)| (i, a.symbol())).collect();
    let expected = vec![(1, "H"), (2, "C"), (3, "N")];
    assert_eq!(d, expected);

    // sort by atomic numbers, hydrogen last
    let numbers: Vec<_> = mol.atomic_numbers().map(|n| std::cmp::Reverse(n)).collect();
    mol.reorder(&numbers);
    let d: Vec<_> = mol.atoms().map(|(i, a)| (i, a.symbol())).collect();
    let expected = vec![(1, "N"), (2, "C"), (3, "H")];
    assert_eq!(d, expected);
}
// test:1 ends here
