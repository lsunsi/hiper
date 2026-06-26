# hiper /ɦɪpɛr/
*hiper*minimal rendering of *hiper*text through rust macros

## Design goals
- Minimal interface that feels like rust.
- Compose templates using functions.
- Zero intermediary allocations.
- Zero procedural macros.
- Zero dependencies.

## What does it look like?
```rust
use hiper::html;

let content = |text: &str| {
    html! { p[] { (text) } }
};

let body = html! {
    body[class = "container"] {
        (content("this looks familiar"))
    }
};

assert_eq!(body(String::new()), r#"
    <body class="container">
        <p>this looks familiar</p>
    </body>
"#);
```

## Inspirations
- [maud](https://github.com/lambda-fairy/maud)
- [horrowshow](https://github.com/Stebalien/horrorshow-rs)
