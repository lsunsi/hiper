macro_rules! assert_render {
    ({ $($tt:tt)* } $lv:literal $ln:literal) => {{
        let mut sv = String::new();
        let mut sn = String::new();
        hiper::html2! { sv; $($tt)* ; };
        hiper::html2! { sn; $($tt)* {} };
        assert_eq!(sv, $lv);
        assert_eq!(sn, $ln);
    }};
}

#[test]
fn tag() {
    assert_render!(
        { a }
        "<a>"
        "<a></a>"
    );
}
