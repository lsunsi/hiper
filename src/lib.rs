#[macro_export]
macro_rules! hiper {
    ($tag:ident[$($k:ident=$v:tt)*]; $($tt:tt)*) => {
        |mut s| {
            s += "<";
            s += stringify!($tag);
            $(
                s += " ";
                s += stringify!($k);
                s += "=";
                s = hiper!(@v $v)(s);
            )*
            s += ">";
            s = hiper!($($tt)*)(s);
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
                s = hiper!(@v $v)(s);
            )*
            s += ">";
            s = hiper!($($c)*)(s);
            s += "</";
            s += stringify!($tag);
            s += ">";
            s = hiper!($($tt)*)(s);
            s
        }
    };
    ($c:literal) => { |s| s + $c };
    (($c:expr)) => { |s| s + $c };

    (@v $v:literal) => { |s| s + "\"" + $v + "\"" };
    (@v ($v:expr)) => { |s| s + "\"" + $v + "\"" };

    () => { |s| s }
}

#[cfg(test)]
mod tests {
    #[test]
    fn tag_void() {
        let h = hiper! { br[]; }(String::new());
        assert_eq!(&h, r#"<br>"#);
    }

    #[test]
    fn tag_void_attr_lit() {
        let h = hiper! { br[id="berre"]; }(String::new());
        assert_eq!(&h, r#"<br id="berre">"#);
    }

    #[test]
    fn tag_void_attr_expr() {
        let id = "berre";
        let h = hiper! { br[id=(id)]; }(String::new());
        assert_eq!(&h, r#"<br id="berre">"#);
    }

    #[test]
    fn tag_void_attr_lit_expr() {
        let href = "/estilo";
        let h = hiper! { link[rel="stylesheet" href=(href)]; }(String::new());
        assert_eq!(&h, r#"<link rel="stylesheet" href="/estilo">"#);
    }

    #[test]
    fn tag_empty() {
        let h = hiper! { a[] {} }(String::new());
        assert_eq!(&h, r#"<a></a>"#);
    }

    #[test]
    fn tag_attr_lit() {
        let h = hiper! { a[href="/sobre"] {} }(String::new());
        assert_eq!(&h, r#"<a href="/sobre"></a>"#);
    }

    #[test]
    fn tag_attr_expr() {
        let href = "/amigos";
        let h = hiper! { a[href=(href)] {} }(String::new());
        assert_eq!(&h, r#"<a href="/amigos"></a>"#);
    }

    #[test]
    fn tag_attr_lit_expr() {
        let target = "_blank";
        let h = hiper! { a[href="/sobre" target=(target)] {} }(String::new());
        assert_eq!(&h, r#"<a href="/sobre" target="_blank"></a>"#);
    }

    #[test]
    fn tag_child_lit() {
        let h = hiper! { a[] { "oiblz" } }(String::new());
        assert_eq!(&h, r#"<a>oiblz</a>"#);
    }

    #[test]
    fn tag_child_expr() {
        let child = "oiblz";
        let h = hiper! { a[] { (child) } }(String::new());
        assert_eq!(&h, r#"<a>oiblz</a>"#);
    }

    #[test]
    fn tag_void_tag_void() {
        let h = hiper! { br[]; link[]; }(String::new());
        assert_eq!(&h, r#"<br><link>"#);
    }

    #[test]
    fn tag_empty_tag_empty() {
        let h = hiper! { a[] {} p[] {} }(String::new());
        assert_eq!(&h, r#"<a></a><p></p>"#);
    }

    #[test]
    fn tag_nested() {
        let h = hiper! { a[] { p[] { br[]; } } }(String::new());
        assert_eq!(&h, r#"<a><p><br></p></a>"#);
    }
}
