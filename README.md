# **/ɦɪpɛr/**
*hiper*minimal rendering of *hiper*text through rust macros

## Design goals
- Minimal interface using familiar syntax. [^1]
- Compose templates using functions. [^2]
- Zero intermediary allocations. [^3]
- Zero procedural macros. [^4]
- Zero dependencies. [^5]

[^1]: The familiarity would come because I tried to copy maud's syntax.
[^2]: I added a syntax that for expressing children with braces to make this more natural coming from common front-end libraries.
[^3]: This means using the previous mentioned syntax guarantees that no intermediary strings will be rendered when you render the whole template.
[^4]: Some people like to avoid procedural macros for many reasons.
[^5]: I added one so the benchmarks would fare better, but I figure it doesn't count. If anyone thinks it does, I can hide it behind a performance feature flag.

## Tag syntax cheatsheet

`br;` if a tag ends with ; it'll be rendered as a void tag.

`div { p { } }` tags children can be nested with braces.

`body #main { }` ids can be ident (or chains of dash joined idents).

`body #"main" { }` ids can be literals too.

`body #(var) { }` id can even be parenthesis wrapped expressions.

`body.bg-red."p-4".(var) { }` classes can be all three as well, but in order.

`body.mx-auto[cond] { }` classes can be conditional.

`input disabled { }` attributes with no values are supported.

`input disabled[!enabled] { }` attributes with no values can be conditional.

`input type="text" hx-get="/" { }` attributes can only be dash joined idents. values can be literal.

`input type="text" value=(var) { }` but values can be expressions as well.

`input type="text" value=[var] { }` and values can be optional.

## Usage by examples

### Basic
```rust
# use hiper::{html, Render};
// Basic
let page = html! {
    head {
        meta charset="UTF-8";
        title { "hiper" }
    }
    body.container {
        h1 #main { "ɦɪpɛr" }
    }
};

let mut s = String::new();
page.render(&mut s);
# assert_eq!(s, r#"<head><meta charset="UTF-8"><title>hiper</title></head><body class="container"><h1 id="main">ɦɪpɛr</h1></body>"#);
```
### Interpolation
```rust
# use hiper::{html, Render};
let page = |charset, title, class, id| html! {
    head {
        meta charset=(charset);
        title { (title) }
    }
    body.(class) {
        h1 #(id) { "ɦɪpɛr" }
    }
};
# let mut s = String::new();
# page("UTF-8", "hiper", "container", "main").render(&mut s);
# assert_eq!(s, r#"<head><meta charset="UTF-8"><title>hiper</title></head><body class="container"><h1 id="main">ɦɪpɛr</h1></body>"#);
```
### Conditional
```rust
# use hiper::{html, Render};
let page = |enabled: bool, value: Option<&'static str>| html! {
    input .glowing[enabled] disabled[!enabled] value=[value];
};
# let mut s = String::new();
# page(true, Some("oiblz")).render(&mut s);
# assert_eq!(s, r#"<input class="glowing" value="oiblz">"#);
# let mut s = String::new();
# page(false, Some("oiblz")).render(&mut s);
# assert_eq!(s, r#"<input disabled value="oiblz">"#);
# let mut s = String::new();
# page(true, None).render(&mut s);
# assert_eq!(s, r#"<input disabled value="oiblz">"#);
# let mut s = String::new();
# page(false, None).render(&mut s);
# assert_eq!(s, r#"<input disabled value="oiblz">"#);
```
### Control flow
```rust
# use hiper::{html, Render};
# let boolean = true;
# let option = Some("oiblz");
# let iterator = 0..1;
# let value = 3;
let page = html! {
    if (boolean) {
        p {}
    }
    if let Some(value) = option {
        p { (value) }
    }
    for (i in iterator) {
        li { (i) }
    }
    match (value) {
        0 => { "zero" },
        _ => { "non-zero" }
    }
};
# let mut s = String::new();
# page.render(&mut s);
# assert_eq!(s, "<p></p><p>oiblz</p><li>0</li>non-zero");
```
### Partials
```rust
# use hiper::{html, Render};
fn head(title: &str) -> impl Render {
    html! {
        head {
            meta charset = "UTF-8";
            title { (title) }
        }
    }
}

fn body(content: impl Render) -> impl Render {
    html! {
        body.container {
            (content)
        }
    }
}

let page = html! {
    head("hiper!");
    body() {
        h1 #main-header { "clearly hyper" }
    }
};
# let mut s = String::new();
# page(&mut s);
# assert_eq!(s, r#"<head><meta charset="UTF-8"><title>hiper!</title></head><body class="container"><h1 id="main-header">clearly hyper</h1></body>"#);
```

## Comparisons

- [maud](https://github.com/lambda-fairy/maud) is the main inspiration for this library.

I like using it and when I realized how simple and elegant it's syntax was I asked myself how close to maud we could get using no proc-macros and having no intermediary allocations. Of course this imposes challenges and trade-offs as well. The answer to the question is all you can see documented here.

TLDR: maud is proc-macros and has intermediary allocations, hiper is decl-macros and has no intermediary allocations.

- [horrorshow](https://github.com/Stebalien/horrorshow-rs) was the validation of concept.

When I did my research I came across this library which gave me a sense my design goals were achievable. The main difference here is that I wanted a syntax closer to maud and a simpler parsing logic. This makes horrorshow the closes library to this one.

TLDR: horrowshow has different syntax than hiper.

- [vy](https://github.com/JonahLund/vy) is a very interesting take.

Because it shares some of the design goals, while also adding more constraints. It actually avoids most of the DSL parsing in order to expose a function-like macro syntax that can still by formatted properly by rustfmt. This improves developer experience immensely.

TLDR: vy has function-like syntax and formatter support. vy is proc-macros, while hiper is decl-macros;

- [askama](https://github.com/askama-rs/askama) is everyone's favorite.

And with good reason. It feels a lot more like regular templating because it uses HTML files instead of macros. It still compiles and gives blazing fast rendering performance and a lot of features to boot.

TLDR: askama uses HTML files instead of inline macros.

###### Thanks for reading all of this.
