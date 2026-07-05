#[test]
fn no_args_no_children() {
    let mut s = String::new();
    let navbar = || hiper::html! { nav {} };
    (hiper::html! { navbar(); })(&mut s);
    assert_eq!(s, "<nav></nav>");
}

#[test]
fn args_no_children() {
    let mut s = String::new();
    let navbar = |title| hiper::html! { nav { title { (title) } } };
    (hiper::html! { navbar("oiblz"); })(&mut s);
    assert_eq!(s, "<nav><title>oiblz</title></nav>");
}

#[test]
fn no_args_and_children() {
    let mut s = String::new();
    let navbar = |children| hiper::html! { nav { title { "oiblz" } (children) } };
    (hiper::html! { navbar() { br; } })(&mut s);
    assert_eq!(s, "<nav><title>oiblz</title><br></nav>");
}

#[test]
fn args_and_children() {
    let mut s = String::new();
    let navbar = |title, children| hiper::html! { nav { title { (title) } (children) } };
    (hiper::html! { navbar("oiblz") { br; } })(&mut s);
    assert_eq!(s, "<nav><title>oiblz</title><br></nav>");
}

#[test]
fn siblings() {
    let mut s = String::new();
    let navbar = || hiper::html! { nav {} };
    (hiper::html! { br; navbar(); p {} })(&mut s);
    assert_eq!(s, "<br><nav></nav><p></p>");
}
