// basic.rs
// :PROPERTIES:
// :header-args: :tangle tests/basic.rs
// :END:

// [[file:~/Workspace/Programming/gchemol-rs/gchemol-core/gchemol-core.note::*basic.rs][basic.rs:1]]
use gchemol_core::*;

#[test]
fn test_molecule_basic() {
    // construct molecule
    let mut mol = Molecule::new("test");
    assert_eq!("test", mol.title());

    let atoms = vec![
        Atom::new("Fe", [1.2; 3]),
        Atom::new("Fe", [1.0; 3]),
        Atom::new("C", [0.0; 3]),
        Atom::new("O", [2.1; 3]),
    ];

    for (i, a) in atoms.into_iter().enumerate() {
        // atom sn counts from 1
        mol.add_atom(i + 1, a);
    }
    assert_eq!(4, mol.natoms());

    mol.add_bond(1, 2, Bond::default());
    mol.add_bond(3, 4, Bond::default());
    assert_eq!(2, mol.nbonds());

    mol.remove_bond(1, 2)
        .expect("failed to remove bond between 1 and 2");
    assert_eq!(1, mol.nbonds());
    mol.add_bond(1, 4, Bond::default());
    assert_eq!(2, mol.nbonds());
    mol.remove_atom(4).expect("failed to remove atom 4");
    assert_eq!(3, mol.natoms());
    assert_eq!(0, mol.nbonds());

    // loop over atoms
    for (_sn, _atom) in mol.atoms() {
        //
    }

    // loop over bonds
    for (_u, _v, _bond) in mol.bonds() {
        //
    }

    // loop over symbols
    for _symbol in mol.symbols() {
        //
    }

    // loop over positions
    for _position in mol.positions() {
        //
    }

    // pick a single atom
    let a = mol.get_atom(1).expect("failed to get atom 1");
    assert_eq!("Fe", a.symbol());
    assert_eq!(1.2, a.position()[0]);

    // edit atom 1
    let a = mol.get_atom_mut(1).expect("failed to get atom 1");
    a.set_symbol("K");
    assert_eq!("K", a.symbol());
}
// basic.rs:1 ends here
