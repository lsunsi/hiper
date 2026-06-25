#[test]
fn index() {
    let hiper = hiper::html! {
        h1[] { "Hello, world!" }
        p[class = "intro"] {
            "This is an example of the "
            a[href="https://github.com/lambda-fairy/maud"] { "Maud" }
            " template language."
        }
    };

    let maud = maud::html! {
        h1 { "Hello, world!" }
        p.intro {
            "This is an example of the "
            a href="https://github.com/lambda-fairy/maud" { "Maud" }
            " template language."
        }
    };

    assert_eq!(hiper(String::new()), maud.0);
}

#[test]
fn getting_started() {
    let name = "Lyra";

    let hiper = hiper::html! {
        p[] { "Hi, " (name) "!" }
    };

    let maud = maud::html! {
        p { "Hi, " (name) "!" }
    };

    assert_eq!(hiper(String::new()), maud.0);
}
