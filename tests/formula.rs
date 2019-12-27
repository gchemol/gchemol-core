// formula.rs
// :PROPERTIES:
// :header-args: :tangle tests/formula.rs
// :END:

// [[file:~/Workspace/Programming/gchemol-rs/gchemol-core/gchemol-core.note::*formula.rs][formula.rs:1]]
#[test]
fn test_formula() {
    use gchemol_core::{Atom, Molecule};

    let mut m = Molecule::new("CH4");
    m.add_atom(1, Atom::new("C", [0.0; 3]));
    m.add_atom(2, Atom::new("H", [0.0; 3]));
    m.add_atom(3, Atom::new("H", [0.0; 3]));
    m.add_atom(4, Atom::new("H", [0.0; 3]));
    m.add_atom(5, Atom::new("H", [0.0; 3]));
    assert_eq!(m.formula(), "CH4");

    let d = m.reduced_symbols();
    assert_eq!(d["C"], 1);
    assert_eq!(d["H"], 4);
}
// formula.rs:1 ends here
