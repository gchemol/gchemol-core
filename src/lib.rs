// header

// [[file:~/Workspace/Programming/gchemol-rs/gchemol-core/gchemol-core.note::*header][header:1]]
//===============================================================================#
//   DESCRIPTION:  molecule object repsented in graph data structure
//
//       OPTIONS:  ---
//  REQUIREMENTS:  ---
//         NOTES:  based on petgraph
//        AUTHOR:  Wenping Guo <ybyygu@gmail.com>
//       LICENCE:  GPL version 3
//       CREATED:  <2018-04-12 Thu 15:48>
//       UPDATED:  <2019-12-26 Thu 11:04>
//===============================================================================#
// header:1 ends here

// imports

// [[file:~/Workspace/Programming/gchemol-rs/gchemol-core/gchemol-core.note::*imports][imports:1]]
use serde::*;

use nxgraph::*;
// imports:1 ends here

// atom

// [[file:~/Workspace/Programming/gchemol-rs/gchemol-core/gchemol-core.note::*atom][atom:1]]
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct Atom {
    /// Element symbol.
    symbol: String,

    /// Atom position.
    position: [f64; 3],
}
// atom:1 ends here

// bond

// [[file:~/Workspace/Programming/gchemol-rs/gchemol-core/gchemol-core.note::*bond][bond:1]]
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct Bond {
    order: f64,
}
// bond:1 ends here

// core

// [[file:~/Workspace/Programming/gchemol-rs/gchemol-core/gchemol-core.note::*core][core:1]]
use bimap::BiHashMap;

type MolGraph = NxGraph<Atom, Bond>;

/// Molecule is the most important data structure in gchemol, which repsents
/// "any singular entity, irrespective of its nature, used to concisely express
/// any type of chemical particle that can exemplify some process: for example,
/// atoms, molecules, ions, etc. can all undergo a chemical reaction". Molecule
/// may have chemical bonds between atoms.
///
/// Reference
/// ---------
/// 1. http://goldbook.iupac.org/M03986.html
/// 2. https://en.wikipedia.org/wiki/Molecular_entity
///
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Molecule {
    /// Molecular name. The default value is `untitled`. This field may be used
    /// to store a registry number or other identifier, instead of a common
    /// name.
    pub name: String,

    // Crystalline lattice for structure using periodic boundary conditions
    // pub lattice: Option<Lattice>,
    /// core data in graph
    graph: MolGraph,

    // mapping: Atom label <=> graph NodeIndex
    mapping: BiHashMap<usize, NodeIndex>,
}

impl Molecule {
    /// get internal node index by atom sn.
    fn node_index(&self, sn: usize) -> NodeIndex {
        *self
            .mapping
            .get_by_left(&sn)
            .expect(&format!("invalid atom sn: {}", sn))
    }

    /// get atom sn  by internal node index.
    fn atom_sn(&self, n: NodeIndex) -> usize {
        *self
            .mapping
            .get_by_right(&n)
            .expect(&format!("invalid NodeIndex: {:?}", n))
    }

    /// Removes atom sn from mapping and returns the associated NodeIndex.
    fn remove_atom_sn(&mut self, sn: usize) -> NodeIndex {
        let (_, n) = self
            .mapping
            .remove_by_left(&sn)
            .expect(&format!("invalid atom sn: {}", sn));
        n
    }
}
// core:1 ends here

// api

// [[file:~/Workspace/Programming/gchemol-rs/gchemol-core/gchemol-core.note::*api][api:1]]
impl Molecule {
    /// Create a new empty molecule with specific name
    pub fn new(name: &str) -> Self {
        Molecule {
            name: name.to_string(),
            ..Default::default()
        }
    }

    /// Add Atom `a` into molecule. If Atom `a` already exists in molecule, then
    /// the associated atom will be updated with `atom`.
    pub fn add_atom(&mut self, a: usize, atom: Atom) {
        if let Some(&n) = self.mapping.get_by_left(&a) {
            self.graph[n] = atom;
        } else {
            let n = self.graph.add_node(atom);
            self.mapping.insert(a, n);
        }
    }

    /// Removes atom `a` from `Molecule`, and returns the removed atom.
    pub fn remove_atom(&mut self, a: usize) -> Atom {
        let n = self.remove_atom_sn(a);
        self.graph.remove_node(n)
    }

    /// Return the number of atoms in the molecule.
    pub fn natoms(&self) -> usize {
        self.graph.number_of_nodes()
    }

    /// Return the number of bonds in the molecule.
    pub fn nbonds(&self) -> usize {
        self.graph.number_of_edges()
    }

    /// Add a bond between atom `a` and atom `b`.
    pub fn add_bond(&mut self, a: usize, b: usize, bond: Bond) {
        let na = self.node_index(a);
        let nb = self.node_index(b);
        self.graph.add_edge(na, nb, bond);
    }

    /// Removes a bond between atom `a` and atom `b`, and returns the removed
    /// `Bond`.
    pub fn remove_bond(&mut self, a: usize, b: usize) -> Bond {
        let na = self.node_index(a);
        let nb = self.node_index(b);
        self.graph.remove_edge(na, nb)
    }

    /// Remove all atoms and bonds.
    pub fn clear(&mut self) {
        self.mapping.clear();
        self.graph.clear();
    }

    pub fn atoms(&self) -> impl Iterator<Item = (usize, &Atom)> {
        self.graph.nodes().map(move |(n, atom)| {
            let sn = self.atom_sn(n);
            (sn, atom)
        })
    }

    pub fn bonds(&self) -> impl Iterator<Item = (usize, usize, &Bond)> {
        self.graph.edges().map(move |(u, v, bond)| {
            let sn1 = self.atom_sn(u);
            let sn2 = self.atom_sn(v);
            (sn1, sn2, bond)
        })
    }
}
// api:1 ends here

// test

// [[file:~/Workspace/Programming/gchemol-rs/gchemol-core/gchemol-core.note::*test][test:1]]
#[test]
fn test() {
    let mut mol = Molecule::new("test");

    for i in 0..5 {
        mol.add_atom(i, Atom::default());
    }
    assert_eq!(mol.natoms(), 5);

    mol.add_bond(1, 2, Bond::default());
    mol.add_bond(2, 3, Bond::default());
    assert_eq!(mol.nbonds(), 2);
    mol.add_bond(2, 1, Bond::default());
    assert_eq!(mol.nbonds(), 2);

    dbg!(mol);
}
// test:1 ends here
