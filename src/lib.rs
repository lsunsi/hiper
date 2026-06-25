#[macro_export]
macro_rules! html {
    ($tag:ident[$($k:ident=$v:tt)*]; $($tt:tt)*) => {
        |mut s| {
            s += "<";
            s += stringify!($tag);
            $(
                s += " ";
                s += stringify!($k);
                s += "=";
                s = $crate::html!(@v $v)(s);
            )*
            s += ">";
            s = $crate::html!($($tt)*)(s);
            s
        }
    };
    ($tag:ident[$($k:ident=$v:tt)*] { $($c:tt)* } $($tt:tt)*) => {
        |mut s| {
            s += "<";
            s += stringify!($tag);
            $(
                s += " ";
                s += stringify!($k);
                s += "=";
                s = $crate::html!(@v $v)(s);
            )*
            s += ">";
            s = $crate::html!($($c)*)(s);
            s += "</";
            s += stringify!($tag);
            s += ">";
            s = $crate::html!($($tt)*)(s);
            s
        }
    };
    ($c:literal $($tt:tt)*) => { |s| $crate::html!($($tt)*)(s + $c) };
    (($c:expr)) => { |s| s + $c };

    (@v $v:literal) => { |s| s + "\"" + $v + "\"" };
    (@v ($v:expr)) => { |s| s + "\"" + $v + "\"" };

    () => { |s| s }
}

#[cfg(test)]
mod tests {
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
}
