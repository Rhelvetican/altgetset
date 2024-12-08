use altgetset::*;

#[derive(Debug, Getter, GetterMut, GetterClone, Setter)]
struct GetSetStruct {
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    field_a: usize,
    #[getset(get, set, get_mut, get_clone)]
    field_b: String,
}

#[test]
fn test_getset() {
    let mut gs = GetSetStruct {
        field_a: 0,
        field_b: "Mark".into(),
    };

    gs.set_field_a(3)
        .get_field_b_mut()
        .push_str(" says Hello, World!");
    dbg!(gs);
}

#[derive(Debug, Getter, GetterMut, GetterClone, Setter)]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
struct GetSetStructPrefix {
    field_a: usize,

    #[getset(skip)]
    /// name
    field_b: String,
}
