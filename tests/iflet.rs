#[test]
fn if_let_some() {
    let mut s = String::new();
    let text = Some("oiblz");
    (hiper::html! { if let Some(a) = text { (a) } })(&mut s);
    assert_eq!(s, "oiblz");
}

#[test]
fn if_let_none() {
    let mut s = String::new();
    let text = None::<&str>;
    (hiper::html! { if let Some(a) = text { p { (a) } } })(&mut s);
    assert_eq!(s, "");
}

#[test]
fn if_let_some_else() {
    let mut s = String::new();
    let text = Some("oi");
    (hiper::html! { if let Some(a) = text { (a) } else { "blz" } })(&mut s);
    assert_eq!(s, "oi");
}

#[test]
fn if_let_none_else() {
    let mut s = String::new();
    let text = None::<&str>;
    (hiper::html! { if let Some(a) = text { p { (a) } } else { "blz" } })(&mut s);
    assert_eq!(s, "blz");
}

#[test]
fn siblings() {
    let mut s = String::new();
    let text = Some("oiblz");
    (hiper::html! { br; if let Some(a) = text { (a) }  p {} })(&mut s);
    assert_eq!(s, "<br>oiblz<p></p>");
}
