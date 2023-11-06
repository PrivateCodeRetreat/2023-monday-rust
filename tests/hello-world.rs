#![cfg_attr(feature = "strict", deny(warnings))]


#[test]
fn die_from_underpopulation() {
    assert_eq!(true, hello_world::next_state());
}
