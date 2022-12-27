// [[file:../gchemol-core.note::*imports][imports:1]]
use crate::Molecule;
// imports:1 ends here

// [[file:../gchemol-core.note::51a9048d][51a9048d]]
fn create_submolecule_from_atoms(mol: &Molecule, atoms: &[usize]) -> Option<Molecule> {
    let nodes: Option<Vec<_>> = atoms.iter().map(|&a| mol.get_node_index(a).copied()).collect();
    let nodes = nodes?;
    let subgraph = mol.graph().subgraph(&nodes);

    Molecule::from_graph(subgraph).into()
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
    pub fn get_sub_molecule(&self, atoms: &[usize]) -> Option<Molecule> {
        create_submolecule_from_atoms(&self, atoms)
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

    let submol = mol.get_sub_molecule(&[1, 2, 3]).unwrap();
    assert_eq!(submol.natoms(), 3);
    assert_eq!(submol.nbonds(), 2);
    assert!(submol.has_bond(1, 2));
    assert!(submol.has_bond(1, 3));
    assert!(!submol.has_bond(2, 3));
}
// cf82e7a7 ends here
