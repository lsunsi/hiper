#[macro_export]
macro_rules! hiper {
    ($tag:ident $ak:ident=$av:literal {}) => {
        |mut s: String| {
            let tag = stringify!($tag);
            s.push('<');
            s.push_str(tag);
            s.push(' ');
            s.push_str(stringify!($ak));
            s.push('=');
            s.push_str(stringify!($av));
            s.push_str("></");
            s.push_str(tag);
            s.push('>');
            s
        }
    };
    ($tag:ident {}) => {
        |mut s: String| {
            let tag = stringify!($tag);
            s.push('<');
            s.push_str(tag);
            s.push_str("></");
            s.push_str(tag);
            s.push('>');
            s
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn tag_bare_empty() {
        let h = hiper! { p {} }(String::new());
        assert_eq!(&h, r#"<p></p>"#);
    }

    #[test]
    fn tag_empty_prop_lit() {
        let h = hiper! { a href="/sobre" {} }(String::new());
        assert_eq!(&h, r#"<a href="/sobre"></a>"#);
    }
}
