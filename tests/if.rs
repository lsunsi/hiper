#[test]
fn if_true() {
    let mut s = String::new();
    (hiper::html! { if (true) { "oiblz" } })(&mut s);
    assert_eq!(s, "oiblz");
}

#[test]
fn if_false() {
    let mut s = String::new();
    (hiper::html! { if (false) { "oiblz" } })(&mut s);
    assert_eq!(s, "");
}

#[test]
fn if_true_else() {
    let mut s = String::new();
    (hiper::html! { if (true) { "oi" } else { "blz" } })(&mut s);
    assert_eq!(s, "oi");
}

#[test]
fn if_false_else() {
    let mut s = String::new();
    (hiper::html! { if (false) { "oi" } else { "blz" } })(&mut s);
    assert_eq!(s, "blz");
}

#[test]
fn siblings() {
    let mut s = String::new();
    (hiper::html! { br; if (true) { "oiblz" }  p {} })(&mut s);
    assert_eq!(s, "<br>oiblz<p></p>");
}
