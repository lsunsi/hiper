#[macro_export]
macro_rules! html {
    ($t:tt$(#$i:tt)?$(.$cs:tt)*[$($kv:tt)*]; $($tt:tt)*) => {
        |mut s| {
            s += "<";
            s += $crate::html!(@t $t);
            s = $crate::html!(@cs $($cs)*)(s);
            $(s = $crate::html!(@i $i)(s);)?
            s = $crate::html!(@kv $($kv)*)(s);
            s += ">";
            s = $crate::html!($($tt)*)(s);
            s
        }
    };
    ($t:tt$(#$i:tt)?$(.$cs:tt)*[$($kv:tt)*] { $($b:tt)* } $($tt:tt)*) => {
        move |mut s| {
            s += "<";
            s += $crate::html!(@t $t);
            s = $crate::html!(@cs $($cs)*)(s);
            $(s = $crate::html!(@i $i)(s))?;
            s = $crate::html!(@kv $($kv)*)(s);
            s += ">";
            s = $crate::html!($($b)*)(s);
            s += "</";
            s += $crate::html!(@t $t);
            s += ">";
            s = $crate::html!($($tt)*)(s);
            s
        }
    };

    (let $i:ident = $e:expr; $($tt:tt)*) => {
        |s| {
            let $i = $e;
            $crate::html!($($tt)*)(s)
        }
    };

    (match ($e:expr) { $($p:pat => {$($b:tt)*}),+ } $($tt:tt)*) => {
        |mut s| {
            match $e { $($p => s = $crate::html!($($b)*)(s),)+ }
            $crate::html!($($tt)*)(s)
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

    (for ($p:pat in $e:expr) { $($body:tt)* } $($tt:tt)*) => {
        |mut s| {
            for $p in $e { s = $crate::html!($($body)*)(s); }
            $crate::html!($($tt)*)(s)
        }
    };

    ($c:literal $($tt:tt)*) => { |s| $crate::html!($($tt)*)($crate::Render::render($c, s)) };
    (($c:expr) $($tt:tt)*) => { move |s| $crate::html!($($tt)*)($crate::Render::render($c, s)) };

    (@t $t:ident) => { stringify!($t) };
    (@t $t:literal) => { $t };

    (@i $i:ident) => {
        |mut s| {
            s += " id=\"";
            s += stringify!($i);
            s += "\"";
            s
        }
    };
    (@i $i:literal) => {
        |mut s| {
            s += " id=\"";
            s += $i;
            s += "\"";
            s
        }
    };
    (@i ($i:expr)) => {
        |mut s| {
            s += " id=\"";
            s += &$i as &str;
            s += "\"";
            s
        }
    };

    (@cs) => { |s| s };
    (@cs $($tt:tt)*) => {
        |mut s| {
            s += " class=\"";
            $(s = $crate::html!(@c $tt)(s) + " ";)*;
            String::pop(&mut s);
            s + "\""
        }
    };
    (@c $c:literal) => { |s| s + $c };
    (@c $c:ident) => { |s| s + stringify!($c) };
    (@c ($c:expr)) => { |s| s + &$c as &str };

    (@c ($($c:expr)*)) => {
        |mut s| {
            let cs: &[&str] = &[$(stringify!($c),)*];
            if !cs.is_empty() {
                s += " class=\"";
                $(
                    s += &$c as &str;
                    s += " ";
                )*
                String::pop(&mut s);
                s += "\"";
            }
            s
        }
    };

    (@kv $k:tt=$v:tt $($tt:tt)*) => {
        |mut s| {
            s += " ";
            s = $crate::html!(@k $k)(s);
            s = $crate::html!(@v $v)(s);
            s = $crate::html!(@kv $($tt)*)(s);
            s
        }
    };
    (@kv $k:tt[$ke:expr] $($tt:tt)*) => {
        |mut s| {
            if $ke {
                s += " ";
                s = $crate::html!(@k $k)(s);
            }
            s = $crate::html!(@kv $($tt)*)(s);
            s
        }
    };
    (@kv) => { |s| s };

    (@k $k:ident) => { |s| s + stringify!($k) };
    (@k $k:literal) => { |s| s + $k };

    (@v ()) => { |s| s };
    (@v $v:literal) => { |s| s + "=" +  "\"" + $v + "\"" };
    (@v ($v:expr)) => { |s| s + "=" + "\"" + &$v as &str + "\"" };

    () => { |s| s }
}
