#[test]
fn children() {
    let mut s = String::new();
    hiper::html2! { s; html { body { main { div { p; } } } } };
    assert_eq!(s, "<html><body><main><div><p></div></main></body></html>");
}

#[test]
fn siblings() {
    let mut s = String::new();
    hiper::html2! { s; a {} a {} a; a; a {} a; };
    assert_eq!(s, "<a></a><a></a><a><a><a></a><a>");
}
