// test_vector3
// :PROPERTIES:
// :header-args: :tangle tests/vector3.rs
// :END:

// [[file:~/Workspace/Programming/gchemol-rs/gchemol-core/gchemol-core.note::*test_vector3][test_vector3:1]]
#[test]
fn test_vector3() {
    use gchemol_core::Vector3f;

    let p = Vector3f::new(1.0, 1.0, 2.0);
    assert_eq!(p.x, 1.0);
    assert_eq!(p.y, 1.0);
    assert_eq!(p.z, 2.0);

    assert_eq!(p[0], 1.0);
    assert_eq!(p[1], 1.0);
    assert_eq!(p[2], 2.0);

    let p: [f64; 3] = p.into();
    assert_eq!(p, [1.0, 1.0, 2.0]);
}
// test_vector3:1 ends here
