#[macro_export]
macro_rules! html {
    ($t:tt$(#$i:tt)?$(.$cs:tt)*[$($kv:tt)*]; $($tt:tt)*) => {
        |s: &mut String| {
            s.push_str("<");
            s.push_str($crate::html!(@t $t));
            $crate::html!(@cs $($cs)*)(s);
            $($crate::html!(@i $i)(s);)?
            $crate::html!(@kv $($kv)*)(s);
            s.push_str(">");
            $crate::html!($($tt)*)(s);
        }
    };
    ($t:tt$(#$i:tt)?$(.$cs:tt)*[$($kv:tt)*] { $($b:tt)* } $($tt:tt)*) => {
        move |s: &mut String| {
            s.push_str("<");
            s.push_str($crate::html!(@t $t));
            $crate::html!(@cs $($cs)*)(s);
            $($crate::html!(@i $i)(s);)?
            $crate::html!(@kv $($kv)*)(s);
            s.push_str(">");
            $crate::html!($($b)*)(s);
            s.push_str("</");
            s.push_str($crate::html!(@t $t));
            s.push_str(">");
            $crate::html!($($tt)*)(s);
        }
    };

    (let $i:ident = $e:expr; $($tt:tt)*) => {
        |s: &mut String| {
            let $i = $e;
            $crate::html!($($tt)*)(s);
        }
    };

    (match ($e:expr) { $($p:pat => {$($b:tt)*}),+ } $($tt:tt)*) => {
        |s: &mut String| {
            match $e { $($p => $crate::html!($($b)*)(s),)+ }
            $crate::html!($($tt)*)(s);
        }
    };

    (if let $cond:pat = $target:ident { $($itt:tt)* } else { $($ett:tt)* } $($tt:tt)*) => {
        |s: &mut String| {
            if let $cond = $target { $crate::html!($($itt)*)(s) } else { $crate::html!($($ett)*)(s) }
            $crate::html!($($tt)*)(s);
        }
    };
    (if let $cond:pat = $target:ident { $($itt:tt)* } $($tt:tt)*) => {
        |s: &mut String| {
            if let $cond = $target { $crate::html!($($itt)*)(s) }
            $crate::html!($($tt)*)(s);
        }
    };

    (if ($icond:expr) { $($ifbody:tt)* } $(else if ($eicond:expr) { $($eibody:tt)* })+ else { $($ebody:tt)* } $($tt:tt)*) => {
        |s: &mut String| {
            (if $icond { $crate::html!($($ifbody)*) }
            $(else if $eicond { $crate::html!($($eibody)*) })+
            else { $crate::html!($($ebody)*) })(s);
            $crate::html!($($tt)*)(s);
        }
    };
    (if ($icond:expr) { $($ifbody:tt)* } $(else if ($eicond:expr) { $($eibody:tt)* })+; $($tt:tt)*) => {
        |s: &mut String| {
            (if $icond { $crate::html!($($ifbody)*)(s); }
            $(else if $eicond { $crate::html!($($eibody)*)(s); })+);
            $crate::html!($($tt)*)(s);
        }
    };
    (if ($icond:expr) { $($ibody:tt)* } else { $($ebody:tt)* } $($tt:tt)*) => {
        |s: &mut String| {
            (if $icond { $crate::html!($($ibody)*) }
            else { $crate::html!($($ebody)*) })(s);
            $crate::html!($($tt)*)(s);
        }
    };
    (if ($icond:expr) { $($ibody:tt)* } $($tt:tt)*) => {
        |s: &mut String| {
            (if $icond { $crate::html!($($ibody)*)(s); });
            $crate::html!($($tt)*)(s);
        }
    };

    (for ($p:pat in $e:expr) { $($body:tt)* } $($tt:tt)*) => {
        |s: &mut String| {
            for $p in $e { $crate::html!($($body)*)(s); }
            $crate::html!($($tt)*)(s);
        }
    };

    ($fn:ident($($args:tt)*); $($tt:tt)*) => {
        move |s: &mut String| {
            $crate::Render::render($fn($($args)*), s);
            $crate::html!($($tt)*)(s);
        }
    };
    ($fn:ident($($args:tt)*) { $($children:tt)* } $($tt:tt)*) => {
        |s: &mut String| {
            $crate::Render::render($fn($($args)*, $crate::html!($($children)*)), s);
            $crate::html!($($tt)*)(s);
        }
    };

    ($c:literal $($tt:tt)*) => { |s: &mut String| {
        $crate::Render::render($c, s);
        $crate::html!($($tt)*)(s);
    }};
    (($c:expr) $($tt:tt)*) => { move |s: &mut String| {
        $crate::Render::render($c, s);
        $crate::html!($($tt)*)(s);
    }};

    (@t $t:ident) => { stringify!($t) };
    (@t $t:literal) => { $t };

    (@i $i:ident) => {
        |s: &mut String| {
            s.push_str(" id=\"");
            s.push_str(stringify!($i));
            s.push_str("\"");
        }
    };
    (@i $i:literal) => {
        |s: &mut String| {
            s.push_str(" id=\"");
            s.push_str($i);
            s.push_str("\"");
        }
    };
    (@i ($i:expr)) => {
        |s: &mut String| {
            s.push_str(" id=\"");
            s.push_str(&$i as &str);
            s.push_str("\"");
        }
    };

    (@cs) => { |s: &mut String| {} };
    (@cs $($tt:tt)*) => {
        |s: &mut String| {
            s.push_str(" class=\"");
            $($crate::html!(@c $tt)(s); s.push_str(" ");)*;
            String::pop(s);
            s.push_str("\"");
        }
    };
    (@c $c:literal) => { |s: &mut String| s.push_str($c) };
    (@c $c:ident) => { |s: &mut String| s.push_str(stringify!($c)) };
    (@c ($c:expr)) => { |s: &mut String| s.push_str(&$c as &str) };

    (@kv $k:tt=$v:tt $($tt:tt)*) => {
        |s: &mut String| {
            s.push_str(" ");
            $crate::html!(@k $k)(s);
            $crate::html!(@v $v)(s);
            $crate::html!(@kv $($tt)*)(s);
        }
    };
    (@kv $k:tt[$ke:expr] $($tt:tt)*) => {
        |s: &mut String| {
            if $ke {
                s.push_str(" ");
                $crate::html!(@k $k)(s);
            }
            $crate::html!(@kv $($tt)*)(s);
        }
    };
    (@kv) => { |s: &mut String| {} };

    (@k $k:ident) => { |s: &mut String| s.push_str(stringify!($k)) };
    (@k $k:literal) => { |s: &mut String| s.push_str($k) };

    (@v ()) => { |s: &mut String| {} };
    (@v $v:literal) => { |s: &mut String| s.push_str(concat!("=", "\"", $v, "\"")) };
    (@v ($v:expr)) => { |s: &mut String| {
        s.push_str("=");
        s.push_str("\"");
        s.push_str(&$v);
        s.push_str("\"");
    }};

    () => { |s: &mut String| {} }
}
