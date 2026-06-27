pub trait Render {
    fn render(self, to: String) -> String;
}

impl Render for &str {
    fn render(self, to: String) -> String {
        crate::escape::write(self, to)
    }
}

impl Render for String {
    fn render(self, to: String) -> String {
        crate::escape::write(&self, to)
    }
}

impl<T: Render> Render for Option<T> {
    fn render(self, to: String) -> String {
        if let Some(t) = self { t.render(to) } else { to }
    }
}

impl<F: FnOnce(String) -> String> Render for F {
    fn render(self, to: String) -> String {
        self(to)
    }
}

impl<T: AsRef<str>> Render for crate::Raw<T> {
    fn render(self, mut to: String) -> String {
        to.push_str(self.0.as_ref());
        to
    }
}
