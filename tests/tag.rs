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

#[test]
fn class_ident_cond() {
    let x = 6;
    assert_render!(
        { a.classe[x > 5].nope[x == 5].klass[x < 9] }
        "<a class=\"classe klass\">"
        "<a class=\"classe klass\"></a>"
    );
}

#[test]
fn class_literal() {
    assert_render!(
        { a."classe"."klass" }
        "<a class=\"classe klass\">"
        "<a class=\"classe klass\"></a>"
    );
}

#[test]
fn class_literal_cond() {
    let class = "klass";
    assert_render!(
        { a."classe"[true]."nope"[class.is_empty()]."klass"[!class.is_empty()] }
        "<a class=\"classe klass\">"
        "<a class=\"classe klass\"></a>"
    );
}

#[test]
fn class_expr() {
    let class = "klass";
    assert_render!(
        { a.("CLASSE".to_lowercase()).(class) }
        "<a class=\"classe klass\">"
        "<a class=\"classe klass\"></a>"
    );
}

#[test]
fn class_expr_cond() {
    let class = "klass";
    assert_render!(
        { a.("CLASSE".to_lowercase())[true].("nope")[class.is_empty()].(class)[!class.is_empty()] }
        "<a class=\"classe klass\">"
        "<a class=\"classe klass\"></a>"
    );
}

#[test]
fn class_mixed() {
    assert_render!(
        { a.b.c[true]."d"."e"[true].("f").("g")[true] }
        "<a class=\"b c d e f g\">"
        "<a class=\"b c d e f g\"></a>"
    );
}

#[test]
fn key_ident_no_value() {
    assert_render!(
        { input checked disabled }
        "<input checked disabled>"
        "<input checked disabled></input>"
    );
}

#[test]
fn key_ident_kebab() {
    assert_render!(
        { input hx-get n-o-o-p }
        "<input hx-get n-o-o-p>"
        "<input hx-get n-o-o-p></input>"
    );
}

#[test]
fn key_ident_cond() {
    let count = 5;
    assert_render!(
        { input checked[count > 3] nope[count != 5] disabled[count < 9] }
        "<input checked disabled>"
        "<input checked disabled></input>"
    );
}

#[test]
fn key_ident_value_literal() {
    assert_render!(
        { a href="/sobre" target="_blank" }
        "<a href=\"/sobre\" target=\"_blank\">"
        "<a href=\"/sobre\" target=\"_blank\"></a>"
    );
}

#[test]
fn key_ident_value_expr() {
    let target = "_BLANK";
    assert_render!(
        { a href=("/sobre") target=(target.to_lowercase()) }
        "<a href=\"/sobre\" target=\"_blank\">"
        "<a href=\"/sobre\" target=\"_blank\"></a>"
    );
}

#[test]
fn key_ident_value_let() {
    let class = "red";
    assert_render!(
        { input type=[Some("text")] nope=[None::<&str>] class=[Some(class)] }
        "<input type=\"text\" class=\"red\">"
        "<input type=\"text\" class=\"red\"></input>"
    );
}
