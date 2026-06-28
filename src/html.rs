#[macro_export]
macro_rules! html {
    ($t:tt[$($k:tt=$v:tt)*]; $($tt:tt)*) => {
        |mut s| {
            s += "<";
            s += $crate::html!(@t $t);
            $(
                s += " ";
                s = $crate::html!(@k $k)(s);
                s = $crate::html!(@v $v)(s);
            )*
            s += ">";
            s = $crate::html!($($tt)*)(s);
            s
        }
    };
    ($t:tt[$($k:tt=$v:tt)*] { $($c:tt)* } $($tt:tt)*) => {
        move |mut s| {
            s += "<";
            s += $crate::html!(@t $t);
            $(
                s += " ";
                s = $crate::html!(@k $k)(s);
                s = $crate::html!(@v $v)(s);
            )*
            s += ">";
            s = $crate::html!($($c)*)(s);
            s += "</";
            s += $crate::html!(@t $t);
            s += ">";
            s = $crate::html!($($tt)*)(s);
            s
        }
    };

    (if let $cond:pat = $target:ident { $($itt:tt)* } else { $($ett:tt)* } $($tt:tt)*) => {
        |s| $crate::html!($($tt)*)(
            (if let $cond = $target { $crate::html!($($itt)*)(s) } else { $crate::html!($($ett)*)(s) })
        )
    };
    (if let $cond:pat = $target:ident { $($itt:tt)* } $($tt:tt)*) => {
        |s| $crate::html!($($tt)*)(
            if let $cond = $target { $crate::html!($($itt)*)(s) } else { s }
        )
    };

    (if ($icond:expr) { $($ifbody:tt)* } $(else if ($eicond:expr) { $($eibody:tt)* })+ else { $($ebody:tt)* } $($tt:tt)*) => {
        |s| {
            $crate::html!($($tt)*)(
                (if $icond { $crate::html!($($ifbody)*) }
                $(else if $eicond { $crate::html!($($eibody)*) })+
                else { $crate::html!($($ebody)*) })(s)
            )
        }
    };
    (if ($icond:expr) { $($ifbody:tt)* } $(else if ($eicond:expr) { $($eibody:tt)* })+; $($tt:tt)*) => {
        |s| {
            $crate::html!($($tt)*)(
                (if $icond { $crate::html!($($ifbody)*) }
                $(else if $eicond { $crate::html!($($eibody)*) })+
                else { |s| s })(s)
            )
        }
    };
    (if ($icond:expr) { $($ibody:tt)* } else { $($ebody:tt)* } $($tt:tt)*) => {
        |s| {
            $crate::html!($($tt)*)(
                (if $icond { $crate::html!($($ibody)*) }
                else { $crate::html!($($ebody)*) })(s)
            )
        }
    };
    (if ($icond:expr) { $($ibody:tt)* } $($tt:tt)*) => {
        |s| {
            $crate::html!($($tt)*)(
                (if $icond { $crate::html!($($ibody)*) } else { |s| s })(s)
            )
        }
    };

    ($c:literal $($tt:tt)*) => { |s| $crate::html!($($tt)*)($crate::Render::render($c, s)) };
    (($c:expr) $($tt:tt)*) => { move |s| $crate::html!($($tt)*)($crate::Render::render($c, s)) };

    (@t $t:ident) => { stringify!($t) };
    (@t $t:literal) => { $t };

    (@k $k:ident) => { |s| s + stringify!($k) };
    (@k $k:literal) => { |s| s + $k };

    (@v ()) => { |s| s };
    (@v $v:literal) => { |s| s + "=" +  "\"" + $v + "\"" };
    (@v ($v:expr)) => { |s| s + "=" + "\"" + &$v as &str + "\"" };

    () => { |s| s }
}
