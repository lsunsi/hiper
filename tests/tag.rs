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

#[test]
fn id_ident() {
    assert_render!(
        { a #idê }
        "<a id=\"idê\">"
        "<a id=\"idê\"></a>"
    );
}

#[test]
fn id_literal() {
    assert_render!(
        { a # "idê" }
        "<a id=\"idê\">"
        "<a id=\"idê\"></a>"
    );
}

#[test]
fn id_expr() {
    assert_render!(
        { a #("IDÊ".to_lowercase()) }
        "<a id=\"idê\">"
        "<a id=\"idê\"></a>"
    );
}

#[test]
fn class_ident() {
    assert_render!(
        { a.classe.klass }
        "<a class=\"classe klass\">"
        "<a class=\"classe klass\"></a>"
    );
}
