#[macro_export]
macro_rules! html {
    ($($tt:tt)*) => {move |s: &mut String| $crate::html2!(s => $($tt)*)}
}

#[macro_export]
macro_rules! html2 {
    ($s:ident => $t:tt$(#$i:tt)?$(.$cs:tt)*[$($kv:tt)*]; $($tt:tt)*) => {{
        $s.push_str("<");
        $s.push_str($crate::html2!(@t $t));
        $crate::html2!(@cs($s) $($cs)*);
        $($crate::html2!(@i($s) $i);)?
        $crate::html2!(@kv($s) $($kv)*);
        $s.push_str(">");
        $crate::html2!($s => $($tt)*);
    }};
    ($s:ident => $t:tt$(#$i:tt)?$(.$cs:tt)*[$($kv:tt)*] { $($b:tt)* } $($tt:tt)*) => {{
        $s.push_str("<");
        $s.push_str($crate::html2!(@t $t));
        $crate::html2!(@cs($s) $($cs)*);
        $($crate::html2!(@i($s) $i);)?
        $crate::html2!(@kv($s) $($kv)*);
        $s.push_str(">");
        $crate::html2!($s => $($b)*);
        $s.push_str("</");
        $s.push_str($crate::html2!(@t $t));
        $s.push_str(">");
        $crate::html2!($s => $($tt)*);
    }};

    ($s:ident => let $i:ident = $e:expr; $($tt:tt)*) => {{
        let $i = $e;
        $crate::html2!($s => $($tt)*);
    }};

    ($s:ident => match ($e:expr) { $($p:pat => {$($b:tt)*}),+ } $($tt:tt)*) => {{
        match $e { $($p => $crate::html2!($s => $($b)*),)+ }
        $crate::html2!($s => $($tt)*);
    }};

    ($s:ident => if let $cond:pat = $target:ident { $($itt:tt)* } else { $($ett:tt)* } $($tt:tt)*) => {{
        if let $cond = $target { $crate::html2!($s => $($itt)*) } else { $crate::html2!($s => $($ett)*) }
        $crate::html2!($s => $($tt)*);
    }};
    ($s:ident => if let $cond:pat = $target:ident { $($itt:tt)* } $($tt:tt)*) => {{
        if let $cond = $target { $crate::html2!($s => $($itt)*) }
        $crate::html2!($s => $($tt)*);
    }};

    ($s:ident => if ($icond:expr) { $($ifbody:tt)* } $(else if ($eicond:expr) { $($eibody:tt)* })+ else { $($ebody:tt)* } $($tt:tt)*) => {{
        (if $icond { $crate::html2!($s => $($ifbody)*) }
        $(else if $eicond { $crate::html2!($s => $($eibody)*) })+
        else { $crate::html2!($s => $($ebody)*) });
        $crate::html2!($s => $($tt)*);
    }};
    ($s:ident => if ($icond:expr) { $($ifbody:tt)* } $(else if ($eicond:expr) { $($eibody:tt)* })+; $($tt:tt)*) => {{
        (if $icond { $crate::html2!($s => $($ifbody)*); }
        $(else if $eicond { $crate::html2!($s => $($eibody)*); })+);
        $crate::html2!($s => $($tt)*);
    }};
    ($s:ident => if ($icond:expr) { $($ibody:tt)* } else { $($ebody:tt)* } $($tt:tt)*) => {{
        (if $icond { $crate::html2!($s => $($ibody)*) }
        else { $crate::html2!($s => $($ebody)*) });
        $crate::html2!($s => $($tt)*);
    }};
    ($s:ident => if ($icond:expr) { $($ibody:tt)* } $($tt:tt)*) => {{
        (if $icond { $crate::html2!($s => $($ibody)*); });
        $crate::html2!($s => $($tt)*);
    }};

    ($s:ident => for ($p:pat in $e:expr) { $($body:tt)* } $($tt:tt)*) => {{
        for $p in $e { $crate::html2!($s => $($body)*); }
        $crate::html2!($s => $($tt)*);
    }};

    ($s:ident => $fn:ident($($args:tt)*); $($tt:tt)*) => {{
        $crate::Render::render($fn($($args)*), $s);
        $crate::html2!($s => $($tt)*);
    }};
    ($s:ident => $fn:ident($($args:tt)*) { $($children:tt)* } $($tt:tt)*) => {{
        $crate::Render::render($fn($($args)*, $crate::html!($($children)*)), $s);
        $crate::html2!($s => $($tt)*);
    }};

    ($s:ident => $c:literal $($tt:tt)*) => {{
        $crate::Render::render($c, $s);
        $crate::html2!($s => $($tt)*);
    }};
    ($s:ident => ($c:expr) $($tt:tt)*) => {{
        $crate::Render::render($c, $s);
        $crate::html2!($s => $($tt)*);
    }};

    (@t $t:ident) => { stringify!($t) };
    (@t $t:literal) => { $t };

    (@i($s:ident) $i:ident) => {{
        $s.push_str(" id=\"");
        $s.push_str(stringify!($i));
        $s.push_str("\"");
    }};
    (@i($s:ident) $i:literal) => {{
        $s.push_str(" id=\"");
        $s.push_str($i);
        $s.push_str("\"");
    }};
    (@i($s:ident) ($i:expr)) => {{
        $s.push_str(" id=\"");
        $s.push_str(&$i as &str);
        $s.push_str("\"");
    }};

    (@cs($s:ident)) => {};
    (@cs($s:ident) $($tt:tt)*) => {{
        $s.push_str(" class=\"");
        $($crate::html2!(@c($s) $tt); $s.push_str(" ");)*;
        String::pop($s);
        $s.push_str("\"");
    }};
    (@c($s:ident) $c:literal) => { $s.push_str($c); };
    (@c($s:ident) $c:ident) => { $s.push_str(stringify!($c)); };
    (@c($s:ident) ($c:expr)) => { $s.push_str(&$c as &str); };

    (@kv($s:ident) $k:tt=$v:tt $($tt:tt)*) => {{
        $s.push_str(" ");
        $crate::html2!(@k($s) $k);
        $crate::html2!(@v($s) $v);
        $crate::html2!(@kv($s) $($tt)*);
    }};
    (@kv($s:ident) $k:tt[$ke:expr] $($tt:tt)*) => {{
        if $ke {
            $s.push_str(" ");
            $crate::html2!(@k($s) $k);
        }
        $crate::html2!(@kv($s) $($tt)*);
    }};
    (@kv($s:ident)) => {};

    (@k($s:ident) $k:ident) => { $s.push_str(stringify!($k)); };
    (@k($s:ident) $k:literal) => { $s.push_str($k); };

    (@v($s:ident) ()) => {};
    (@v($s:ident) $v:literal) => { $s.push_str(concat!("=", "\"", $v, "\"")); };
    (@v($s:ident) ($v:expr)) => {{
        $s.push_str("=");
        $s.push_str("\"");
        $s.push_str(&$v);
        $s.push_str("\"");
    }};

    ($s:ident =>) => {}
}
