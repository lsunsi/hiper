#[test]
fn let_decl() {
    let mut s = String::new();
    let arr = ["oi", "blz"];
    (hiper::html! { let len = arr.len(); (len) "!" })(&mut s);
    assert_eq!(s, "2!");
}
