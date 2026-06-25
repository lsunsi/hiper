#[macro_export]
macro_rules! hiper {
    ($tag:ident $ak:ident=($av:expr) {}) => {
        |s: String| {
            s + "<"
                + stringify!($tag)
                + " "
                + stringify!($ak)
                + "=\""
                + $av
                + "\"></"
                + stringify!($tag)
                + ">"
        }
    };
    ($tag:ident $ak:ident=$av:literal {}) => {
        |s: String| {
            s + "<"
                + stringify!($tag)
                + " "
                + stringify!($ak)
                + "="
                + stringify!($av)
                + "></"
                + stringify!($tag)
                + ">"
        }
    };
    ($tag:ident {}) => {
        |s: String| s + "<" + stringify!($tag) + "></" + stringify!($tag) + ">"
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn tag_bare_empty() {
        let h = hiper! { a {} }(String::new());
        assert_eq!(&h, r#"<a></a>"#);
    }

    #[test]
    fn tag_empty_prop_lit() {
        let h = hiper! { a href="/sobre" {} }(String::new());
        assert_eq!(&h, r#"<a href="/sobre"></a>"#);
    }

    #[test]
    fn tag_empty_prop_var() {
        let href = "/amigos";
        let h = hiper! { a href=(href) {} }(String::new());
        assert_eq!(&h, r#"<a href="/amigos"></a>"#);
    }
}
