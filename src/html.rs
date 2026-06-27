#[macro_export]
macro_rules! html {
    ($tag:ident[$($k:ident=$v:tt)*]; $($tt:tt)*) => {
        |mut s| {
            s += "<";
            s += stringify!($tag);
            $(
                s += " ";
                s += stringify!($k);
                s += "=";
                s = $crate::html!(@v $v)(s);
            )*
            s += ">";
            s = $crate::html!($($tt)*)(s);
            s
        }
    };
    ($tag:ident[$($k:ident=$v:tt)*] { $($c:tt)* } $($tt:tt)*) => {
        move |mut s| {
            s += "<";
            s += stringify!($tag);
            $(
                s += " ";
                s += stringify!($k);
                s += "=";
                s = $crate::html!(@v $v)(s);
            )*
            s += ">";
            s = $crate::html!($($c)*)(s);
            s += "</";
            s += stringify!($tag);
            s += ">";
            s = $crate::html!($($tt)*)(s);
            s
        }
    };

    (if let $cond:pat = $target:ident { $($itt:tt)* } else { $($ett:tt)* }) => {
        |s| if let $cond = $target { $crate::html!($($itt)*)(s) } else { $crate::html!($($ett)*)(s) }
    };
    (if let $cond:pat = $target:ident { $($itt:tt)* }) => {
        |s| if let $cond = $target { $crate::html!($($itt)*)(s) } else { s }
    };
    (if ($cond:expr) { $($itt:tt)* } else { $($ett:tt)* }) => {
        if $cond { $crate::html!($($itt)*) } else { $crate::html!($($ett)*) }
    };
    (if ($cond:expr) { $($itt:tt)* }) => {
        if $cond { $crate::html!($($itt)*) } else { |s| s }
    };

    ($c:literal $($tt:tt)*) => { |s| $crate::html!($($tt)*)($crate::Render::render($c, s)) };
    (($c:expr) $($tt:tt)*) => { move |s| $crate::html!($($tt)*)($crate::Render::render($c, s)) };

    (@v $v:literal) => { |s| s + "\"" + $v + "\"" };
    (@v ($v:expr)) => { |s| s + "\"" + $v + "\"" };

    () => { |s| s }
}
