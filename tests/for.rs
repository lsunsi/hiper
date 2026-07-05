#[test]
fn nonempty() {
    let mut s = String::new();
    (hiper::html! { for (i in 0..2) { li { (i) } } })(&mut s);
    assert_eq!(s, "<li>0</li><li>1</li>");
}

#[test]
fn empty() {
    let mut s = String::new();
    (hiper::html! { for (i in 0..0) { li { (i) } } })(&mut s);
    assert_eq!(s, "");
}

#[test]
fn siblings() {
    let mut s = String::new();
    (hiper::html! { br; for (i in ["oi"]) { (i) } p {} })(&mut s);
    assert_eq!(s, "<br>oi<p></p>");
}
