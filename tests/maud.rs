#[test]
fn index() {
    let hiper = hiper::html! {
        h1[] { "Hello, world!" }
        p[class = "intro"] {
            "This is an example of the "
            a[href="https://github.com/lambda-fairy/maud"] { "Maud" }
            " template language."
        }
    }(String::new());

    let maud = maud::html! {
        h1 { "Hello, world!" }
        p.intro {
            "This is an example of the "
            a href="https://github.com/lambda-fairy/maud" { "Maud" }
            " template language."
        }
    }
    .0;

    assert_eq!(hiper, maud);
}
