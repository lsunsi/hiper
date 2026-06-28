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

mod getting_started {
    #[test]
    fn index() {
        let name = "Lyra";
        assert_tmpl!({ p { "Hi, " (name) "!" } }, { p[] { "Hi, " (name) "!" } });
    }
}

mod text_and_escaping {
    #[test]
    fn text() {
        assert_tmpl!({ "Oatmeal, are you crazy?" }, { "Oatmeal, are you crazy?" });
    }

    #[test]
    fn raw_strings() {
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
    fn escaping_and_preescaped() {
        use hiper::Raw;
        use maud::PreEscaped;
        assert_tmpl!(
            { "<script>alert(\"XSS\")</script>"(PreEscaped("<script>alert(\"XSS\")</script>",)) },
            { "<script>alert(\"XSS\")</script>"(Raw("<script>alert(\"XSS\")</script>")) }
        );
    }

    #[test]
    fn the_doctype_constant() {
        assert_tmpl!({ (maud::DOCTYPE) }, { (hiper::DOCTYPE) });
    }
}

mod elements_and_attributes {
    #[test]
    fn elements_with_contents() {
        assert_tmpl!({
            h1 { "Poem" }
            p {
                strong { "Rock," }
                " you are a rock."
            }
        }, {
            h1[] { "Poem" }
            p[] {
                strong[] { "Rock," }
                " you are a rock."
            }
        });
    }

    #[test]
    fn void_elements() {
        assert_tmpl!({
            link rel="stylesheet" href="poetry.css";
            p {
                "Rock, you are a rock."
                br;
                "Gray, you are gray,"
                br;
                "Like a rock, which you are."
                br;
                "Rock."
            }
        }, {
            link[rel="stylesheet" href="poetry.css"];
            p[] {
                "Rock, you are a rock."
                br[];
                "Gray, you are gray,"
                br[];
                "Like a rock, which you are."
                br[];
                "Rock."
            }
        });
    }

    #[test]
    fn custom_elements_and_data_attributes() {
        assert_tmpl!({
            article data-index="12345" {
                h1 { "My blog" }
                tag-cloud { "pinkie pie pony cute" }
            }
        }, {
            article["data-index"="12345"] {
                h1[] { "My blog" }
                "tag-cloud"[] { "pinkie pie pony cute" }
            }
        });
    }

    #[test]
    fn non_empty_attributes() {
        assert_tmpl!({
            ul {
                li {
                    a href="about:blank" { "Apple Bloom" }
                }
                li class="lower-middle" {
                    "Sweetie Belle"
                }
                li dir="rtl" {
                    "Scootaloo "
                    small { "(also a chicken)" }
                }
            }
        }, {
            ul[] {
                li[] {
                    a[href="about:blank"] { "Apple Bloom" }
                }
                li[class="lower-middle"] {
                    "Sweetie Belle"
                }
                li[dir="rtl"] {
                    "Scootaloo "
                    small[] { "(also a chicken)" }
                }
            }
        });
    }

    #[test]
    fn empty_attributes() {
        assert_tmpl!({
            form {
                input type="checkbox" name="cupcakes" checked;
                " "
                label for="cupcakes" { "Do you like cupcakes?" }
            }
        }, {
            form[] {
                input[type="checkbox" name="cupcakes" checked=()];
                " "
                label[for="cupcakes"] { "Do you like cupcakes?" }
            }
        });
    }

    #[test]
    fn classes_and_ids_base() {
        assert_tmpl!({
            input #cannon .big.scary.bright-red type="button" value="Launch Party Cannon";
        }, {
            input #cannon .big.scary."bright-red"[type="button" value="Launch Party Cannon"];
        });
    }

    #[test]
    fn classes_and_ids_quoted() {
        assert_tmpl!({
            div."col-sm-2" { "Bootstrap column!" }
        }, {
            div."col-sm-2"[] { "Bootstrap column!" }
        });
    }

    #[test]
    fn implicit_div_elements() {
        // no support for implicit divs
        assert_tmpl!({
            #main {
                "Main content!"
                .tip { "Storing food in a refrigerator can make it 20% cooler." }
            }
        }, {
            div #main[] {
                "Main content!"
                div.tip[] { "Storing food in a refrigerator can make it 20% cooler." }
            }
        });
    }
}

mod splices_and_toggles {
    #[test]
    fn base() {
        let best_pony = "Pinkie Pie";
        let numbers = [1, 2, 3, 4];
        assert_tmpl!({
            p { "Hi, " (best_pony) "!" }
            p {
                "I have " (numbers.len()) " numbers, "
                "and the first one is " (numbers[0])
            }
        }, {
            p[] { "Hi, " (best_pony) "!" }
            p[] {
                "I have " (numbers.len()) " numbers, "
                "and the first one is " (numbers[0])
            }
        });
    }

    #[test]
    fn block() {
        struct Foo;
        struct FooFmt;

        impl Foo {
            fn time(self) -> FooFmt {
                FooFmt
            }
        }

        impl FooFmt {
            fn format(self, _: &str) -> &'static str {
                "oiblz"
            }
        }

        fn something_convertible_to_foo() -> Foo {
            Foo
        }

        assert_tmpl!({
            p {
                ({
                    let f: Foo = something_convertible_to_foo();
                    f.time().format("%H%Mh")
                })
            }
        }, {
            p[] {
                ({
                    let f: Foo = something_convertible_to_foo();
                    f.time().format("%H%Mh")
                })
            }
        });
    }

    #[test]
    fn splices_in_attributes_base() {
        let secret_message = "Surprise!";
        assert_tmpl!({
            p title=(secret_message) {
                "Nothing to see here, move along."
            }
        }, {
            p[title=(secret_message)] {
                "Nothing to see here, move along."
            }
        });
    }

    #[test]
    fn splices_in_attributes_concat() {
        const GITHUB: &str = "https://github.com";
        assert_tmpl!({
            a href={ (GITHUB) "/lambda-fairy/maud" } {
                "Fork me on GitHub"
            }
        }, {
            a[href=(GITHUB.to_string() + "/lambda-fairy/maud")] {
                "Fork me on GitHub"
            }
        });
    }

    #[test]
    fn splices_in_classes_and_ids() {
        let name = "rarity";
        let severity = "critical";
        assert_tmpl!({
            aside #(name) {
                p.{ "color-" (severity) } { "This is the worst! Possible! Thing!" }
            }
        }, {
            aside #(name)[] {
                p.("color-".to_string() + severity )[] { "This is the worst! Possible! Thing!" }
            }
        });
    }

    #[test]
    fn what_can_be_spliced() {
        let post = "<p>Pre-escaped</p>";
        assert_tmpl!({
            h1 { "My super duper blog post" }
            (maud::PreEscaped(post))
        }, {
            h1[] { "My super duper blog post" }
            (hiper::Raw(post))
        });
    }

    #[test]
    fn toggles_base() {
        let allow_editing = true;
        assert_tmpl!({
            p contenteditable[allow_editing] {
                "Edit me, I "
                em { "dare" }
                " you."
            }
        }, {
            p[contenteditable[allow_editing]] {
                "Edit me, I "
                em[] { "dare" }
                " you."
            }
        });
    }

    #[test]
    #[ignore]
    fn toggles_classes() {
        unimplemented!();
        // let cuteness = 95;
        // assert_tmpl!({
        //     p.cute[cuteness > 50] { "Squee!" }
        // }, {
        //     p.cute[cuteness > 50] { "Squee!" }
        // });
    }

    #[test]
    #[ignore]
    fn optional_attributes_with_values() {
        unimplemented!();
        // assert_tmpl!({
        //     p title=[Some("Good password")] { "Correct horse" }

        //     @let value = Some(42);
        //     input value=[value];

        //     @let title: Option<&str> = None;
        //     p title=[title] { "Battery staple" }
        // }, {
        //     p title=[Some("Good password")] { "Correct horse" }

        //     @let value = Some(42);
        //     input value=[value];

        //     @let title: Option<&str> = None;
        //     p title=[title] { "Battery staple" }
        // });
    }
}

mod control_structures {
    #[test]
    fn branching_with_if_and_else_base() {
        #[derive(PartialEq)]
        enum Princess {
            Celestia,
            Luna,
        }

        let user = Princess::Celestia;
        assert_tmpl!({
            @if user == Princess::Luna {
                h1 { "Super secret woona to-do list" }
                ul {
                    li { "Nuke the Crystal Empire" }
                    li { "Kick a puppy" }
                    li { "Evil laugh" }
                }
            } @else if user == Princess::Celestia {
                p { "Sister, please stop reading my diary. Its rude." }
            } @else {
                p { "Nothing to see here; move along." }
            }
        }, {
            if (user == Princess::Luna) {
                h1[] { "Super secret woona to-do list" }
                ul[] {
                    li[] { "Nuke the Crystal Empire" }
                    li[] { "Kick a puppy" }
                    li[] { "Evil laugh" }
                }
            } else if (user == Princess::Celestia) {
                p[] { "Sister, please stop reading my diary. Its rude." }
            } else {
                p[] { "Nothing to see here; move along." }
            }
        });
    }

    #[test]
    fn branching_with_if_and_else_let() {
        let user = Some("Pinkie Pie");
        assert_tmpl!({
            p {
                "Hello, "
                @if let Some(name) = user {
                    (name)
                } @else {
                    "stranger"
                }
                "!"
            }
        }, {
            p[] {
                "Hello, "
                if let Some(name) = user {
                    (name)
                } else {
                    "stranger"
                }
                "!"
            }
        });
    }

    #[test]
    fn looping_with_for() {
        let names = ["Applejack", "Rarity", "Fluttershy"];
        assert_tmpl!({
            p { "My favorite ponies are:" }
            ol {
                @for name in &names {
                    li { (name) }
                }
            }
        }, {
            p[] { "My favorite ponies are:" }
            ol[] {
                for (name in names) {
                    li[] { (name) }
                }
            }
        });
    }

    #[test]
    fn declaring_variables_with_let() {
        let names = ["Applejack", "Rarity", "Fluttershy"];
        assert_tmpl!({
            @for name in &names {
                @let first_letter = name.chars().next().unwrap();
                p {
                    "The first letter of "
                    b { (name) }
                    " is "
                    b { (first_letter) }
                    "."
                }
            }
        }, {
            for (name in &names) {
                let first_letter = name.chars().next().unwrap();
                p[] {
                    "The first letter of "
                    b[] { (*name) }
                    " is "
                    b[] { (first_letter.to_string()) }
                    "."
                }
            }
        });
    }

    #[test]
    fn matching_with_match() {
        #[allow(dead_code, reason = "used to match")]
        enum Princess {
            Celestia,
            Luna,
            Cadance,
            TwilightSparkle,
        }
        let user = Princess::Celestia;
        assert_tmpl!({
            @match user {
                Princess::Luna => {
                    h1 { "Super secret woona to-do list" }
                    ul {
                        li { "Nuke the Crystal Empire" }
                        li { "Kick a puppy" }
                        li { "Evil laugh" }
                    }
                },
                Princess::Celestia => {
                    p { "Sister, please stop reading my diary. Its rude." }
                },
                _ => p { "Nothing to see here; move along." }
            }
        }, {
            match (user) {
                Princess::Luna => {
                    h1[] { "Super secret woona to-do list" }
                    ul[] {
                        li[] { "Nuke the Crystal Empire" }
                        li[] { "Kick a puppy" }
                        li[] { "Evil laugh" }
                    }
                },
                Princess::Celestia => {
                    p[] { "Sister, please stop reading my diary. Its rude." }
                },
                _ => { p[] { "Nothing to see here; move along." } }
            }
        });
    }
}

mod partials {
    #[test]
    fn index() {
        mod m {
            fn header(page_title: &str) -> maud::Markup {
                maud::html! {
                    (maud::DOCTYPE)
                    meta charset="utf-8";
                    title { (page_title) }
                }
            }

            fn footer() -> maud::Markup {
                maud::html! {
                    footer {
                        a href="rss.atom" { "RSS Feed" }
                    }
                }
            }

            pub(super) fn page(title: &str, greeting_box: maud::Markup) -> maud::Markup {
                maud::html! {
                    (header(title))
                    h1 { (title) }
                    (greeting_box)
                    (footer())
                }
            }
        }

        mod h {
            fn header(page_title: &str) -> impl hiper::Render {
                hiper::html! {
                    (hiper::DOCTYPE)
                    meta[charset="utf-8"];
                    title[] { (page_title) }
                }
            }

            fn footer() -> impl hiper::Render {
                hiper::html! {
                    footer[] {
                        a[href="rss.atom"] { "RSS Feed" }
                    }
                }
            }

            pub(super) fn page(title: &str, greeting: impl hiper::Render) -> impl hiper::Render {
                hiper::html! {
                    header(title);
                    h1[] { (title) }
                    (greeting)
                    footer();
                }
            }
        }

        assert_tmpl!(
            {
                (m::page(
                    "Hello!",
                    maud::html! {
                        div { "Greetings, Maud." }
                    },
                ))
            },
            {
                (h::page(
                    "Hello!",
                    hiper::html! {
                        div[] { "Greetings, Maud." }
                    },
                ))
            }
        );

        let maud_page = m::page;
        let hiper_page = h::page;

        assert_tmpl!(
            {
                (maud_page(
                    "Hello!",
                    maud::html! {
                        div { "Greetings, Maud." }
                    },
                ))
            },
            {
                hiper_page("Hello!") {
                    div[] { "Greetings, Maud." }
                }
            }
        );
    }
}
