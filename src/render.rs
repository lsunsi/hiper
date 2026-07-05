/// trait used for rendering values into html
pub trait Render {
    fn render(self, to: &mut String);
}

impl Render for &str {
    fn render(self, to: &mut String) {
        crate::escape::render(self, to);
    }
}

impl Render for String {
    fn render(self, to: &mut String) {
        crate::escape::render(&self, to);
    }
}

impl<T: Render> Render for Option<T> {
    fn render(self, to: &mut String) {
        if let Some(t) = self {
            t.render(to);
        }
    }
}

impl<F: FnOnce(&mut String)> Render for F {
    fn render(self, to: &mut String) {
        self(to);
    }
}

impl<T: AsRef<str>> Render for crate::Raw<T> {
    fn render(self, to: &mut String) {
        to.push_str(self.0.as_ref());
    }
}

macro_rules! impl_render_through_itoap {
    ($($type:ty)+) => {$(
        impl Render for $type {
            fn render(self, to: &mut String) {
                itoap::write_to_string(to, self)
            }
        }
    )+};
}

impl_render_through_itoap!(u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 usize isize);
