macro_rules! assert_tmpl {
    ({$($maud:tt)+}, {$($hiper:tt)+}) => {{
        let maud = maud::html! {$($maud)+};
        let hiper = hiper::html! {$($hiper)+};
        assert_eq!(maud.0, hiper(String::new()));
    }};
}

#[test]
fn index() {
    assert_tmpl!({
        h1 { "Hello, world!" }
        p.intro {
            "This is an example of the "
            a href="https://github.com/lambda-fairy/maud" { "Maud" }
            " template language."
        }
    }, {
        h1[] { "Hello, world!" }
        p[class = "intro"] {
            "This is an example of the "
            a[href="https://github.com/lambda-fairy/maud"] { "Maud" }
            " template language."
        }
    });
}

#[test]
fn getting_started() {
    let name = "Lyra";
    assert_tmpl!({ p { "Hi, " (name) "!" } }, { p[] { "Hi, " (name) "!" } });
}

#[test]
fn text_escaping_text() {
    assert_tmpl!({ "Oatmeal, are you crazy?" }, { "Oatmeal, are you crazy?" });
}

#[test]
fn text_escaping_raw_strings() {
    assert_tmpl!({
        pre {
            r#"
                Rocks, these are my rocks.
                Sediments make me sedimental.
                Smooth and round,
                Asleep in the ground.
                Shades of brown
                And gray.
            "#
        }
    }, {
        pre[] {
            r#"
                Rocks, these are my rocks.
                Sediments make me sedimental.
                Smooth and round,
                Asleep in the ground.
                Shades of brown
                And gray.
            "#
        }
    });
}

#[test]
fn text_escaping_preescaped() {
    assert_tmpl!(
        { "<script>alert(\"XSS\")</script>"(maud::PreEscaped("<script>alert(\"XSS\")</script>")) },
        { "<script>alert(\"XSS\")</script>"(hiper::Raw("<script>alert(\"XSS\")</script>")) }
    );
}

#[test]
fn text_escaping_doctype() {
    assert_tmpl!({ (maud::DOCTYPE) }, { (hiper::DOCTYPE) });
}
