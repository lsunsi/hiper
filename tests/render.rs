#[test]
fn str() {
    let mut s = String::new();
    (hiper::html! { "oiblz" })(&mut s);
    assert_eq!(s, "oiblz");
}

#[test]
fn str_escape() {
    let mut s = String::new();
    (hiper::html! { "<p>'\"bl&z\"'</p>" })(&mut s);
    assert_eq!(s, "&lt;p&gt;&apos;&quot;bl&amp;z&quot;&apos;&lt;/p&gt;");
}

#[test]
fn string() {
    let mut s = String::new();
    (hiper::html! { (String::from("oiblz")) })(&mut s);
    assert_eq!(s, "oiblz");
}

#[test]
fn string_escape() {
    let mut s = String::new();
    (hiper::html! { (String::from("<p>'\"bl&z\"'</p>")) })(&mut s);
    assert_eq!(s, "&lt;p&gt;&apos;&quot;bl&amp;z&quot;&apos;&lt;/p&gt;");
}

#[test]
fn numbers() {
    let mut s = String::new();
    (hiper::html! { (0u8)(1i8)(2u16)(3i16)(4u32)(5i32)(6u64)(7i64)(8u128)(9i128)(0usize)(1isize) })(
        &mut s,
    );
    assert_eq!(s, "012345678901");
}

#[test]
fn none() {
    let mut s = String::new();
    (hiper::html! { (None::<&str>) })(&mut s);
    assert_eq!(s, "");
}

#[test]
fn some_render() {
    let mut s = String::new();
    (hiper::html! { (Some("oiblz")) })(&mut s);
    assert_eq!(s, "oiblz");
}

#[test]
fn raw() {
    let mut s = String::new();
    (hiper::html! { (hiper::Raw("<br>")) })(&mut s);
    assert_eq!(s, "<br>");
}
