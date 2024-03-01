// [[file:../gchemol-core.note::ff231cb5][ff231cb5]]
use crate::common::*;
use crate::Molecule;
use crate::PropertyStore;
// ff231cb5 ends here

// [[file:../gchemol-core.note::82f7facb][82f7facb]]
use crate::Atom;
use std::collections::HashSet;

/// A defined linked collection of atoms or a single atom within a
/// molecular entity.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct AtomGroup {
    atoms: HashSet<NodeIndex>,

    /// Arbitrary property stored in key-value pair. Key is a string
    /// type, but it is the responsibility of the user to correctly
    /// interpret the value.
    pub properties: PropertyStore,
}

impl AtomGroup {
    fn new(mol: &Molecule, atoms: &[usize]) -> Option<Self> {
        let atoms = atoms.iter().filter_map(|&n| mol.get_node_index(n).copied()).collect();
        Self { atoms, ..Default::default() }.into()
    }

    fn get_atoms<'a>(&'a self, mol: &'a Molecule) -> impl Iterator<Item = (usize, &'a Atom)> + 'a {
        self.atoms.iter().map(move |&i| (mol.atom_sn(i), &mol.graph[i]))
    }
}

/// Atom groups related methods
impl Molecule {
    /// Define a new atom group with `group_name` using atoms in
    /// `group`. Old group with the same name will be overwrote.
    pub fn define_group(&mut self, group_name: &str, group: &[usize]) -> Option<AtomGroup> {
        let group = AtomGroup::new(self, group)?;
        self.groups.insert(group_name.to_string(), group)
    }

    /// Removes an atom group `group_name` from `Molecule`, returning
    /// the `AtomGroup` with `group_name` if it previously in the
    /// `Molecule`.
    pub fn remove_group(&mut self, group_name: &str) -> Option<AtomGroup> {
        self.groups.remove(group_name)
    }

    /// Returns true if `Molecule` contains atom group `group_name`.
    pub fn has_group(&self, group_name: &str) -> bool {
        self.groups.contains_key(group_name)
    }

    /// Renames an atom group `old_name` with `new_name`.
    pub fn rename_group(&mut self, old_name: &str, new_name: &str) -> Option<()> {
        let group = self.groups.remove(old_name)?;
        self.groups.insert(new_name.to_string(), group);
        Some(())
    }

    /// Accesses the atom group in `group_name`.
    pub fn get_group(&self, group_name: &str) -> Option<&AtomGroup> {
        self.groups.get(group_name)
    }

    /// Mut access to the atom group in `group_name`.
    pub fn get_group_mut(&mut self, group_name: &str) -> Option<&mut AtomGroup> {
        self.groups.get_mut(group_name)
    }

    /// Gets atoms in group `group_name`.
    pub fn get_atoms_in_group(&self, group_name: &str) -> Option<impl Iterator<Item = (usize, &Atom)>> {
        let group = self.get_group(group_name)?;
        Some(group.get_atoms(self))
    }
}
// 82f7facb ends here

// [[file:../gchemol-core.note::51a9048d][51a9048d]]
fn create_submolecule_from_atoms<'a>(mol: &Molecule, atoms: impl IntoIterator<Item = &'a usize>) -> Option<Molecule> {
    let atoms: Vec<_> = atoms.into_iter().copied().collect();
    let nodes: Option<Vec<_>> = atoms.iter().copied().map(|a| mol.get_node_index(a).copied()).collect();
    let nodes = nodes?;
    let graph = mol.graph().subgraph(&nodes);

    Molecule::from_graph_raw(graph, atoms).into()
}
// 51a9048d ends here

// [[file:../gchemol-core.note::687744ec][687744ec]]
use gchemol_graph::petgraph::algo;

/// High level topology structure of `Molecule`.
impl Molecule {
    /// Return the shortest distance counted in number of chemical bonds between
    /// two atoms. Return None if they are not connected.
    pub fn nbonds_between(&self, sn1: usize, sn2: usize) -> Option<usize> {
        let graph = self.graph.raw_graph();
        let node1 = self.node_index(sn1);
        let node2 = self.node_index(sn2);

        let path = algo::astar(graph, node1, |finish| finish == node2, |_| 1, |_| 0);
        if let Some((n, _)) = path {
            Some(n)
        } else {
            None
        }
    }

    /// Return the shortest path between two atoms. Return None if they are not
    /// connected.
    ///
    /// # Panics
    ///
    /// * panic if there is no atom associated with `sn1` or `sn2`
    pub fn path_between(&self, sn1: usize, sn2: usize) -> Option<Vec<usize>> {
        let graph = self.graph.raw_graph();
        let node1 = self.node_index(sn1);
        let node2 = self.node_index(sn2);

        let path = algo::astar(graph, node1, |finish| finish == node2, |_| 1, |_| 0);
        if let Some((_, p)) = path {
            // convert node indices to atom serial numbers
            let sns: Vec<_> = p.into_iter().map(|n| self.atom_sn(n)).collect();
            Some(sns)
        } else {
            None
        }
    }

    /// Return all directly bonded atoms with `a`
    pub fn connected(&self, a: usize) -> impl Iterator<Item = usize> + '_ {
        let node = self.node_index(a);
        self.graph.neighbors(node).map(move |b| self.atom_sn(b))
    }

    /// Return a sub molecule induced by `atoms` in parent
    /// molecule. Return None if atom serial numbers are
    /// invalid. Return an empty Molecule if `atoms` empty.
    ///
    /// # NOTE
    /// * The sub molecule shares the same numbering system with its parent.
    pub fn get_sub_molecule<'a>(&self, atoms: impl IntoIterator<Item = &'a usize>) -> Option<Molecule> {
        create_submolecule_from_atoms(&self, atoms)
    }

    /// Return a shallow connectivity graph without copying atom/bond data
    pub fn bond_graph(&self) -> NxGraph<usize, usize> {
        let mut result_g = NxGraph::new();

        // add nodes from atom serial numbers
        let node_map: std::collections::HashMap<_, _> = self.numbers().map(|n| (n, result_g.add_node(n))).collect();

        // add edges from chemical bonds
        for (i, j, _) in self.bonds() {
            let src = node_map[&i];
            let tar = node_map[&j];
            result_g.add_edge(src, tar, 1);
        }

        result_g
    }

    /// Break molecule into multiple fragments based on its bonding
    /// connectivity. Return molecules whole connected by bonds
    /// without periodic lattice. The atom numbers in fragments will
    /// be the same as in their parent.
    pub fn fragmented(&self) -> impl Iterator<Item = Self> + '_ {
        self.graph().connected_components_node_indices().map(|nodes| {
            let numbers: Vec<_> = nodes.iter().map(|&n| self.atom_sn(n)).collect();
            let g = self.graph().subgraph(&nodes);
            Molecule::from_graph_raw(g, numbers)
        })
    }

    /// Return the number of fragments based on bonding connectivity.
    pub fn nfragments(&self) -> usize {
        self.graph().connected_components().count()
    }

    /// Return all atoms that connected in the same fragment as atom
    /// `i`.
    pub fn connected_fragment_atoms(&self, i: usize) -> impl Iterator<Item = usize> + '_ {
        let node = self.node_index(i);
        self.graph().node_connected_component(node).map(|n| self.atom_sn(n))
    }
}
// 687744ec ends here

// [[file:../gchemol-core.note::cf82e7a7][cf82e7a7]]
#[test]
fn test_topo_path() {
    use crate::Atom;

    // CH4 molecule
    let mut mol = Molecule::from_database("CH4");
    mol.rebond();

    let n = mol.nbonds_between(1, 2);
    assert_eq!(n, Some(1));
    let n = mol.nbonds_between(2, 3);
    assert_eq!(n, Some(2));

    let p = mol.path_between(1, 2);
    assert_eq!(p, Some(vec![1, 2]));

    let empty = mol.get_sub_molecule(&[]);
    assert!(empty.is_some());
    assert_eq!(empty.unwrap().natoms(), 0);

    let submol = mol.get_sub_molecule(&[1, 2, 4]).unwrap();
    assert_eq!(submol.natoms(), 3);
    assert_eq!(submol.nbonds(), 2);
    assert!(submol.has_bond(1, 2));
    assert!(submol.has_bond(1, 4));
    assert!(!submol.has_bond(2, 4));
    assert_eq!(mol.get_distance(2, 4), submol.get_distance(2, 4));
    assert_eq!(mol.get_distance(1, 4), submol.get_distance(1, 4));
    assert_eq!(mol.get_distance(1, 2), submol.get_distance(1, 2));

    let a4_parent = mol.get_atom_unchecked(4);
    let a4_child = submol.get_atom_unchecked(4);
    assert_eq!(a4_parent.position(), a4_child.position());

    // test fragments
    let mol1 = Molecule::from_database("CH4");
    let mut mol2 = mol1.clone();
    let n = mol.natoms();
    // move mol1 away
    mol.translate([10.0, 0.0, 0.0]);
    for (i, a) in mol.atoms() {
        mol2.add_atom(i + n, a.clone());
    }
    mol2.rebond();
    let frags = mol2.fragmented().collect_vec();

    assert_eq!(frags.len(), 2);
    assert_eq!(frags[0].formula(), "CH4");
    assert_eq!(frags[1].formula(), "CH4");

    // atom numbers in each fragment should be the same as in `mol2`
    let numbers: std::collections::HashSet<_> = frags.iter().map(|frag| frag.numbers()).flatten().collect();
    assert_eq!(numbers.len(), mol2.natoms());
}
// cf82e7a7 ends here
