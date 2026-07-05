#[test]
fn children() {
    let mut s = String::new();
    (hiper::html! { html { body { main { div { p; } } } } })(&mut s);
    assert_eq!(s, "<html><body><main><div><p></div></main></body></html>");
}

#[test]
fn siblings() {
    let mut s = String::new();
    (hiper::html! { a {} a {} a; a; a {} a; })(&mut s);
    assert_eq!(s, "<a></a><a></a><a><a><a></a><a>");
}
