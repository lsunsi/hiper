pub trait Render {
    fn render(self, to: String) -> String;
}

impl Render for &str {
    fn render(self, mut to: String) -> String {
        to.push_str(self);
        to
    }
}

impl Render for String {
    fn render(self, mut to: String) -> String {
        to.push_str(&self);
        to
    }
}

impl<T: Render> Render for Option<T> {
    fn render(self, to: String) -> String {
        if let Some(t) = self { t.render(to) } else { to }
    }
}
