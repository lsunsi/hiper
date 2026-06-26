pub trait Render {
    fn render(self, to: String) -> String;
}

impl Render for &str {
    fn render(self, mut to: String) -> String {
        to.push_str(self);
        to
    }
}
