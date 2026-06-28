pub use hiper::{Render, html};

#[test]
fn render_str() {
    let h = html! { "oiblz" };
    assert_eq!(h(String::new()), r#"oiblz"#);
}

#[test]
fn render_str_escape() {
    let h = html! { r#"<p>'"bl&z"'</p>"# };
    assert_eq!(
        h(String::new()),
        r#"&lt;p&gt;&apos;&quot;bl&amp;z&quot;&apos;&lt;/p&gt;"#
    );
}

#[test]
fn render_string() {
    let h = html! { (String::from("oiblz")) };
    assert_eq!(h(String::new()), r#"oiblz"#);
}

#[test]
fn render_string_escape() {
    let h = html! { (String::from(r#"<p>'"bl&z"'</p>"#)) };
    assert_eq!(
        h(String::new()),
        r#"&lt;p&gt;&apos;&quot;bl&amp;z&quot;&apos;&lt;/p&gt;"#
    );
}

#[test]
fn render_numbers() {
    let h = html! { (0u8)(1i8)(2u16)(3i16)(4u32)(5i32)(6u64)(7i64)(8u128)(9i128)(0usize)(1isize) };
    assert_eq!(h(String::new()), r#"012345678901"#);
}

#[test]
fn render_none() {
    let h = html! { (None::<&str>) };
    assert_eq!(h(String::new()), r#""#);
}

#[test]
fn render_some_render() {
    let h = html! { (Some("oiblz")) };
    assert_eq!(h(String::new()), r#"oiblz"#);
}

#[test]
fn expr_as_str() {
    let h = html! { ((1 + 2).to_string()) };
    assert_eq!(h(String::new()), r#"3"#);
}

#[test]
fn tag_void() {
    let h = html! { br[]; };
    assert_eq!(h(String::new()), r#"<br>"#);
}

#[test]
fn tag_void_attr_lit() {
    let h = html! { br[id="berre"]; };
    assert_eq!(h(String::new()), r#"<br id="berre">"#);
}

#[test]
fn tag_void_attr_expr() {
    let id = "berre";
    let h = html! { br[id=(id)]; };
    assert_eq!(h(String::new()), r#"<br id="berre">"#);
}

#[test]
fn tag_void_attr_lit_expr() {
    let href = "/estilo";
    let h = html! { link[rel="stylesheet" href=(href)]; };
    assert_eq!(
        h(String::new()),
        r#"<link rel="stylesheet" href="/estilo">"#
    );
}

#[test]
fn tag_empty() {
    let h = html! { a[] {} };
    assert_eq!(h(String::new()), r#"<a></a>"#);
}

#[test]
fn tag_attr_lit() {
    let h = html! { a[href="/sobre"] {} };
    assert_eq!(h(String::new()), r#"<a href="/sobre"></a>"#);
}

#[test]
fn tag_attr_expr() {
    let href = "/amigos";
    let h = html! { a[href=(href)] {} };
    assert_eq!(h(String::new()), r#"<a href="/amigos"></a>"#);
}

#[test]
fn tag_attr_lit_expr() {
    let target = "_blank";
    let h = html! { a[href="/sobre" target=(target)] {} };
    assert_eq!(h(String::new()), r#"<a href="/sobre" target="_blank"></a>"#);
}

#[test]
fn tag_child_lit() {
    let h = html! { a[] { "oiblz" } };
    assert_eq!(h(String::new()), r#"<a>oiblz</a>"#);
}

#[test]
fn tag_child_expr() {
    let child = "oiblz";
    let h = html! { a[] { (child) } };
    assert_eq!(h(String::new()), r#"<a>oiblz</a>"#);
}

#[test]
fn tag_void_tag_void() {
    let h = html! { br[]; link[]; };
    assert_eq!(h(String::new()), r#"<br><link>"#);
}

#[test]
fn tag_empty_tag_empty() {
    let h = html! { a[] {} p[] {} };
    assert_eq!(h(String::new()), r#"<a></a><p></p>"#);
}

#[test]
fn tag_nested() {
    let h = html! { a[] { p[] { br[]; } } };
    assert_eq!(h(String::new()), r#"<a><p><br></p></a>"#);
}

#[test]
fn tag_literal_tag_literal() {
    let h = html! { a[] {} "oi" br[]; "blz" };
    assert_eq!(h(String::new()), r#"<a></a>oi<br>blz"#);
}

#[test]
fn tag_literal_expr_expr_literal() {
    let (name, surname) = ("carlos", "marcos");
    let h = html! { br[]; "oi " (name) (surname) "!" };
    assert_eq!(h(String::new()), r#"<br>oi carlosmarcos!"#);
}

#[test]
fn tag_nested_render_expr() {
    let render_user = |name: &'static str| html! { p[] { (name) } };
    let h = html! { div[] { (render_user("carlos")) } };
    assert_eq!(h(String::new()), r#"<div><p>carlos</p></div>"#);
}

#[test]
fn tag_if_true() {
    let h = html! { div[] { if (true) { p[] { "oiblz" } } } };
    assert_eq!(h(String::new()), r#"<div><p>oiblz</p></div>"#);
}

#[test]
fn tag_if_false() {
    let h = html! { div[] { if (false) { p[] { "oiblz" } } } };
    assert_eq!(h(String::new()), r#"<div></div>"#);
}

#[test]
fn tag_if_true_else() {
    let h = html! { div[] { if (true) { p[] { "oi" } } else { span[] { "blz" } } } };
    assert_eq!(h(String::new()), r#"<div><p>oi</p></div>"#);
}

#[test]
fn tag_if_false_else() {
    let h = html! { div[] { if (false) { p[] { "oi" } } else { span[] { "blz" } } } };
    assert_eq!(h(String::new()), r#"<div><span>blz</span></div>"#);
}

#[test]
fn tag_if_true_else_if() {
    let h = html! { p[] { if (true) { "oi" } else if (false) { "blz" }; } };
    assert_eq!(h(String::new()), r#"<p>oi</p>"#);
}

#[test]
fn tag_if_false_else_if_true() {
    let h = html! { p[] { if (false) { "oi" } else if (true) { "blz" }; } };
    assert_eq!(h(String::new()), r#"<p>blz</p>"#);
}

#[test]
fn tag_if_false_else_if_false_else_if_true() {
    let h = html! { p[] { if (false) { "oi" } else if (false) { "blz" } else if (true) { "!" }; } };
    assert_eq!(h(String::new()), r#"<p>!</p>"#);
}

#[test]
fn tag_if_false_else_if_true_else() {
    let h = html! { p[] { if (false) { "oi" } else if (true) { "blz" } else { "!" } } };
    assert_eq!(h(String::new()), r#"<p>blz</p>"#);
}

#[test]
fn tag_if_false_else_if_false_else() {
    let h = html! { p[] { if (false) { "oi" } else if (false) { "blz" } else { "!" } } };
    assert_eq!(h(String::new()), r#"<p>!</p>"#);
}

#[test]
fn tag_if_false_else_if_false_else_if_true_else() {
    let h = html! { p[] { if (false) { "oi" } else if (false) { "blz" } else if (true) { "!" } else { "." } } };
    assert_eq!(h(String::new()), r#"<p>!</p>"#);
}

#[test]
fn tag_if_false_else_if_false_else_if_false_else() {
    let h = html! { p[] { if (false) { "oi" } else if (false) { "blz" } else if (false) { "!" } else { "." } } };
    assert_eq!(h(String::new()), r#"<p>.</p>"#);
}

#[test]
fn tag_if_let_true() {
    let text = Some("oiblz");
    let h = html! { div[] { if let Some(a) = text { p[] { (a) } } } };
    assert_eq!(h(String::new()), r#"<div><p>oiblz</p></div>"#);
}

#[test]
fn tag_if_let_false() {
    let text = None::<&str>;
    let h = html! { div[] { if let Some(a) = text { p[] { (a) } } } };
    assert_eq!(h(String::new()), r#"<div></div>"#);
}

#[test]
fn tag_if_let_true_else() {
    let text = Some("oi");
    let h = html! { div[] { if let Some(a) = text { p[] { (a) } } else { "blz" } } };
    assert_eq!(h(String::new()), r#"<div><p>oi</p></div>"#);
}

#[test]
fn tag_if_let_false_else() {
    let text = None::<&str>;
    let h = html! { div[] { if let Some(a) = text { p[] { (a) } } else { "blz" } } };
    assert_eq!(h(String::new()), r#"<div>blz</div>"#);
}

#[test]
fn tag_if_tag() {
    let h = html! { p[] {} if (true) { "oiblz" } br[]; };
    assert_eq!(h(String::new()), r#"<p></p>oiblz<br>"#);
}

#[test]
fn tag_if_else_tag() {
    let h = html! { p[] {} if (true) { "oi" } else if (false) { "blz" }; br[]; };
    assert_eq!(h(String::new()), r#"<p></p>oi<br>"#);
}

#[test]
fn tag_if_else_if_tag() {
    let h = html! { p[] {} if (true) { "oi" } else { "blz" } br[]; };
    assert_eq!(h(String::new()), r#"<p></p>oi<br>"#);
}

#[test]
fn tag_if_else_if_else_tag() {
    let h = html! { p[] {} if (true) { "oi" } else if (false) { "blz" } else { "!" } br[]; };
    assert_eq!(h(String::new()), r#"<p></p>oi<br>"#);
}

#[test]
fn tag_if_let_tag() {
    let text = Some("oiblz");
    let h = html! { p[] {} if let Some(a) = text { (a) } br[]; };
    assert_eq!(h(String::new()), r#"<p></p>oiblz<br>"#);
}

#[test]
fn tag_if_let_else_tag() {
    let text = None::<&str>;
    let h = html! { p[] {} if let Some(a) = text { (a) } else { "oiblz" } br[]; };
    assert_eq!(h(String::new()), r#"<p></p>oiblz<br>"#);
}

#[test]
fn tag_literal() {
    assert_eq!(
        html! { "web-comp"[] {} "comp-web"[]; }(String::new()),
        r#"<web-comp></web-comp><comp-web>"#
    );
}

#[test]
fn tag_attr_key_literal() {
    assert_eq!(
        html! { div["hx-get" = "/"] {} br["hx-post" = "/"]; }(String::new()),
        r#"<div hx-get="/"></div><br hx-post="/">"#
    );
}

#[test]
fn tag_empty_attr() {
    assert_eq!(
        html! { br[checked=()]; p[checked=()] {} }(String::new()),
        r#"<br checked><p checked></p>"#
    );
}

#[test]
fn tag_for_tag() {
    assert_eq!(
        html! { ul[] { for (i in 0..2) { li[] { (i) } } } }(String::new()),
        r#"<ul><li>0</li><li>1</li></ul>"#
    );
}

#[test]
fn tag_for_tag_tag() {
    assert_eq!(
        html! { br[]; for (i in 0..2) { li[] { (i) } } p[] {} }(String::new()),
        r#"<br><li>0</li><li>1</li><p></p>"#
    );
}

#[test]
fn tag_toggle_attr_true() {
    let editable = true;
    assert_eq!(
        html! { p[contenteditable[editable]] {} }(String::new()),
        r#"<p contenteditable></p>"#
    );
}

#[test]
fn tag_toggle_attr_false() {
    let editable = false;
    assert_eq!(
        html! { p[contenteditable[editable]] {} }(String::new()),
        r#"<p></p>"#
    );
}

#[test]
fn tag_let_tag() {
    assert_eq!(
        html! { p[] { let len = ["oi", "blz"].len(); (len) "!" } }(String::new()),
        r#"<p>2!</p>"#
    );
}
