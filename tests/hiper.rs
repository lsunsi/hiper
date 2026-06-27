pub use hiper::{Render, html};

#[test]
fn render_str() {
    let h = html! { "oiblz" }(String::new());
    assert_eq!(&h, r#"oiblz"#);
}

#[test]
fn render_str_escape() {
    let h = html! { r#"<p>'"bl&z"'</p>"# }(String::new());
    assert_eq!(&h, r#"&lt;p&gt;&apos;&quot;bl&amp;z&quot;&apos;&lt;/p&gt;"#);
}

#[test]
fn render_string() {
    let h = html! { (String::from("oiblz")) }(String::new());
    assert_eq!(&h, r#"oiblz"#);
}

#[test]
fn render_string_escape() {
    let h = html! { (String::from(r#"<p>'"bl&z"'</p>"#)) }(String::new());
    assert_eq!(&h, r#"&lt;p&gt;&apos;&quot;bl&amp;z&quot;&apos;&lt;/p&gt;"#);
}

#[test]
fn render_none() {
    let h = html! { (None::<&str>) }(String::new());
    assert_eq!(&h, r#""#);
}

#[test]
fn render_some_render() {
    let h = html! { (Some("oiblz")) }(String::new());
    assert_eq!(&h, r#"oiblz"#);
}

#[test]
fn expr_as_str() {
    let h = html! { ((1 + 2).to_string()) }(String::new());
    assert_eq!(&h, r#"3"#);
}

#[test]
fn tag_void() {
    let h = html! { br[]; }(String::new());
    assert_eq!(&h, r#"<br>"#);
}

#[test]
fn tag_void_attr_lit() {
    let h = html! { br[id="berre"]; }(String::new());
    assert_eq!(&h, r#"<br id="berre">"#);
}

#[test]
fn tag_void_attr_expr() {
    let id = "berre";
    let h = html! { br[id=(id)]; }(String::new());
    assert_eq!(&h, r#"<br id="berre">"#);
}

#[test]
fn tag_void_attr_lit_expr() {
    let href = "/estilo";
    let h = html! { link[rel="stylesheet" href=(href)]; }(String::new());
    assert_eq!(&h, r#"<link rel="stylesheet" href="/estilo">"#);
}

#[test]
fn tag_empty() {
    let h = html! { a[] {} }(String::new());
    assert_eq!(&h, r#"<a></a>"#);
}

#[test]
fn tag_attr_lit() {
    let h = html! { a[href="/sobre"] {} }(String::new());
    assert_eq!(&h, r#"<a href="/sobre"></a>"#);
}

#[test]
fn tag_attr_expr() {
    let href = "/amigos";
    let h = html! { a[href=(href)] {} }(String::new());
    assert_eq!(&h, r#"<a href="/amigos"></a>"#);
}

#[test]
fn tag_attr_lit_expr() {
    let target = "_blank";
    let h = html! { a[href="/sobre" target=(target)] {} }(String::new());
    assert_eq!(&h, r#"<a href="/sobre" target="_blank"></a>"#);
}

#[test]
fn tag_child_lit() {
    let h = html! { a[] { "oiblz" } }(String::new());
    assert_eq!(&h, r#"<a>oiblz</a>"#);
}

#[test]
fn tag_child_expr() {
    let child = "oiblz";
    let h = html! { a[] { (child) } }(String::new());
    assert_eq!(&h, r#"<a>oiblz</a>"#);
}

#[test]
fn tag_void_tag_void() {
    let h = html! { br[]; link[]; }(String::new());
    assert_eq!(&h, r#"<br><link>"#);
}

#[test]
fn tag_empty_tag_empty() {
    let h = html! { a[] {} p[] {} }(String::new());
    assert_eq!(&h, r#"<a></a><p></p>"#);
}

#[test]
fn tag_nested() {
    let h = html! { a[] { p[] { br[]; } } }(String::new());
    assert_eq!(&h, r#"<a><p><br></p></a>"#);
}

#[test]
fn tag_literal_tag_literal() {
    let h = html! { a[] {} "oi" br[]; "blz" }(String::new());
    assert_eq!(&h, r#"<a></a>oi<br>blz"#);
}

#[test]
fn tag_literal_expr_expr_literal() {
    let (name, surname) = ("carlos", "marcos");
    let h = html! { br[]; "oi " (name) (surname) "!" }(String::new());
    assert_eq!(&h, r#"<br>oi carlosmarcos!"#);
}

#[test]
fn tag_nested_render_expr() {
    let render_user = |name: &'static str| html! { p[] { (name) } };
    let h = html! { div[] { (render_user("carlos")) } }(String::new());
    assert_eq!(&h, r#"<div><p>carlos</p></div>"#);
}

#[test]
fn tag_if_true() {
    let h = html! { div[] { if (true) { p[] { "oiblz" } } } }(String::new());
    assert_eq!(&h, r#"<div><p>oiblz</p></div>"#);
}

#[test]
fn tag_if_false() {
    let h = html! { div[] { if (false) { p[] { "oiblz" } } } }(String::new());
    assert_eq!(&h, r#"<div></div>"#);
}

#[test]
fn tag_if_true_else() {
    let h = html! { div[] { if (true) { p[] { "oi" } } else { span[] { "blz" } } } }(String::new());
    assert_eq!(&h, r#"<div><p>oi</p></div>"#);
}

#[test]
fn tag_if_false_else() {
    let h =
        html! { div[] { if (false) { p[] { "oi" } } else { span[] { "blz" } } } }(String::new());
    assert_eq!(&h, r#"<div><span>blz</span></div>"#);
}
