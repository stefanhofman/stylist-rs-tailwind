use stylist_tailwind_macros::tw;

#[test]
fn single_works() {
    let a = tw!("ml-4");
    assert_eq!(a, "margin-left: 1rem;");
}

#[test]
fn multiple_works() {
    let a = tw!("ml-4 p-4");
    assert_eq!(a, "margin-left: 1rem; padding: 1rem;");
}
