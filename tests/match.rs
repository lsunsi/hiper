#[test]
fn tag_match_tag() {
    let mut s = String::new();

    enum Carlos {
        Roberto,
        Marcos,
    }

    let carlos = Carlos::Marcos;
    (hiper::html! {
        "-"
        match (carlos) {
            Carlos::Roberto => { "roberto" },
            Carlos::Marcos => { strong { "marcos" } }
        }
        "!"
        match (Carlos::Roberto) {
            Carlos::Roberto => { "roberto" },
            Carlos::Marcos => { strong { "marcos" } }
        }
        "?"
    })(&mut s);

    assert_eq!(s, "-<strong>marcos</strong>!roberto?");
}
