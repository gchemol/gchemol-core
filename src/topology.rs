// imports

// [[file:~/Workspace/Programming/gchemol-rs/gchemol-core/gchemol-core.note::*imports][imports:1]]
use crate::Molecule;
// imports:1 ends here

// api

// [[file:~/Workspace/Programming/gchemol-rs/gchemol-core/gchemol-core.note::*api][api:1]]
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

    /// Return the shortest path between two atoms. Return None if them are not
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
}
// api:1 ends here

// test

// [[file:~/Workspace/Programming/gchemol-rs/gchemol-core/gchemol-core.note::*test][test:1]]
#[test]
fn test_topo_path() {
    use crate::Atom;

    // CH4 molecule
    let atom1 = Atom::new("C", [-0.90203687, 0.62555259, 0.0081889]);
    let atom2 = Atom::new("H", [-0.54538244, -0.38325741, 0.0081889]);
    let atom3 = Atom::new("H", [-0.54536403, 1.12995078, 0.88184041]);
    let atom4 = Atom::new("H", [-0.54536403, 1.12995078, -0.8654626]);
    let atom5 = Atom::new("H", [-1.97203687, 0.62556577, 0.0081889]);

    let mut mol = Molecule::from_atoms(vec![atom1, atom2, atom3, atom4, atom5]);
    mol.rebond();

    let n = mol.nbonds_between(1, 2);
    assert_eq!(n, Some(1));
    let n = mol.nbonds_between(2, 3);
    assert_eq!(n, Some(2));

    let p = mol.path_between(1, 2);
    assert_eq!(p, Some(vec![1, 2]));
}
// test:1 ends here
