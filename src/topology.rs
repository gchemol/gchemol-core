// [[file:../gchemol-core.note::ff231cb5][ff231cb5]]
use crate::common::*;
use crate::Molecule;
// ff231cb5 ends here

// [[file:../gchemol-core.note::51a9048d][51a9048d]]
fn create_submolecule_from_atoms(mol: &Molecule, atoms: &[usize]) -> Option<Molecule> {
    let nodes: Option<Vec<_>> = atoms.iter().map(|&a| mol.get_node_index(a).copied()).collect();
    let nodes = nodes?;
    let graph = mol.graph().subgraph(&nodes);

    Molecule::from_graph_raw(graph, atoms.into_iter().copied()).into()
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
    pub fn get_sub_molecule(&self, atoms: &[usize]) -> Option<Molecule> {
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
    /// without periodic lattice
    pub fn fragmented(&self) -> impl Iterator<Item = Self> + '_ {
        self.graph().connected_components().map(|g| Molecule::from_graph(g))
    }

    /// Return the number of fragments based on bonding connectivity.
    pub fn nfragments(&self) -> usize {
        self.graph().connected_components().count()
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
}
// cf82e7a7 ends here
