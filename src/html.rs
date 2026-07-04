#[macro_export]
macro_rules! html {
    ($($tt:tt)*) => {move |s: &mut String| $crate::html2!(s; $($tt)*)}
}

#[macro_export]
macro_rules! html2 {
    ($s:ident;
        $tag:ident
        $(# $idi:ident)?
        $(# $idl:literal)?
        $(# ($ide:expr))?
        $(. $classi:ident $([$classicond:expr])?)*
        $(. $classl:literal $([$classlcond:expr])?)*
        $(. ($classe:expr) $([$classecond:expr])?)*
        $($key:ident $([$keycond:expr])? $(= $($valuel:literal)? $(($valuee:expr))? $([$valuecond:expr])?)?)*
        ;
        $($sibling:tt)*
    ) => {
        $s.push_str(concat!('<', stringify!($tag)));
        $crate::render_id!($s; $($idi)* $($idl)* $($ide)*);
        $crate::render_classes!($s; @0 $($classi, $($classicond)*;)* $($classl, $($classlcond)*;)* $($classe, $($classecond)*;)*);
        $($crate::render_keyvalue!($s; $key $($keycond)*$(; $($valuel)* $($valuee)* $(; $valuecond)*)*);)*
        $s.push('>');
        $crate::html2!($s; $($sibling)*);
    };
    ($s:ident;
        $tag:ident
        $(# $idi:ident)?
        $(# $idl:literal)?
        $(# ($ide:expr))?
        $(. $classi:ident $([$classicond:expr])?)*
        $(. $classl:literal $([$classlcond:expr])?)*
        $(. ($classe:expr) $([$classecond:expr])?)*
        $($key:ident $([$keycond:expr])? $(= $($valuel:literal)? $(($valuee:expr))? $([$valuecond:expr])?)?)*
        { $($child:tt)* }
        $($sibling:tt)*
    ) => {
        $s.push_str(concat!('<', stringify!($tag)));
        $crate::render_id!($s; $($idi)* $($idl)* $($ide)*);
        $crate::render_classes!($s; @0 $($classi, $($classicond)*;)* $($classl, $($classlcond)*;)* $($classe, $($classecond)*;)*);
        $($crate::render_keyvalue!($s; $key $($keycond)*$(; $($valuel)* $($valuee)* $(; $valuecond)*)*);)*
        $s.push('>');
        $crate::html2!($s; $($child)*);
        $s.push_str(concat!("</", stringify!($tag), '>'));
        $crate::html2!($s; $($sibling)*);
    };
    ($s:ident; $t:tt$(#$i:tt)?$(.$cs:tt)*[$($kv:tt)*]; $($tt:tt)*) => {{
        $s.push_str("<");
        $s.push_str($crate::html2!(@t $t));
        $crate::html2!($s; @cs $($cs)*);
        $($crate::html2!($s; @i $i);)?
        $crate::html2!($s; @kv $($kv)*);
        $s.push_str(">");
        $crate::html2!($s; $($tt)*);
    }};
    ($s:ident; $t:tt$(#$i:tt)?$(.$cs:tt)*[$($kv:tt)*] { $($b:tt)* } $($tt:tt)*) => {{
        $s.push_str("<");
        $s.push_str($crate::html2!(@t $t));
        $crate::html2!($s; @cs $($cs)*);
        $($crate::html2!($s; @i $i);)?
        $crate::html2!($s; @kv $($kv)*);
        $s.push_str(">");
        $crate::html2!($s; $($b)*);
        $s.push_str("</");
        $s.push_str($crate::html2!(@t $t));
        $s.push_str(">");
        $crate::html2!($s; $($tt)*);
    }};

    ($s:ident; let $i:ident = $e:expr; $($tt:tt)*) => {{
        let $i = $e;
        $crate::html2!($s; $($tt)*);
    }};

    ($s:ident; match ($e:expr) { $($p:pat => {$($b:tt)*}),+ } $($tt:tt)*) => {{
        match $e { $($p => $crate::html2!($s; $($b)*),)+ }
        $crate::html2!($s; $($tt)*);
    }};

    ($s:ident; if let $cond:pat = $target:ident { $($itt:tt)* } else { $($ett:tt)* } $($tt:tt)*) => {{
        if let $cond = $target { $crate::html2!($s; $($itt)*) } else { $crate::html2!($s; $($ett)*) }
        $crate::html2!($s; $($tt)*);
    }};
    ($s:ident; if let $cond:pat = $target:ident { $($itt:tt)* } $($tt:tt)*) => {{
        if let $cond = $target { $crate::html2!($s; $($itt)*) }
        $crate::html2!($s; $($tt)*);
    }};

    ($s:ident; if ($icond:expr) { $($ifbody:tt)* } $(else if ($eicond:expr) { $($eibody:tt)* })+ else { $($ebody:tt)* } $($tt:tt)*) => {{
        (if $icond { $crate::html2!($s; $($ifbody)*) }
        $(else if $eicond { $crate::html2!($s; $($eibody)*) })+
        else { $crate::html2!($s; $($ebody)*) });
        $crate::html2!($s; $($tt)*);
    }};
    ($s:ident; if ($icond:expr) { $($ifbody:tt)* } $(else if ($eicond:expr) { $($eibody:tt)* })+; $($tt:tt)*) => {{
        (if $icond { $crate::html2!($s; $($ifbody)*); }
        $(else if $eicond { $crate::html2!($s; $($eibody)*); })+);
        $crate::html2!($s; $($tt)*);
    }};
    ($s:ident; if ($icond:expr) { $($ibody:tt)* } else { $($ebody:tt)* } $($tt:tt)*) => {{
        (if $icond { $crate::html2!($s; $($ibody)*) }
        else { $crate::html2!($s; $($ebody)*) });
        $crate::html2!($s; $($tt)*);
    }};
    ($s:ident; if ($icond:expr) { $($ibody:tt)* } $($tt:tt)*) => {{
        (if $icond { $crate::html2!($s; $($ibody)*); });
        $crate::html2!($s; $($tt)*);
    }};

    ($s:ident; for ($p:pat in $e:expr) { $($body:tt)* } $($tt:tt)*) => {{
        for $p in $e { $crate::html2!($s; $($body)*); }
        $crate::html2!($s; $($tt)*);
    }};

    ($s:ident; $fn:ident($($args:tt)*); $($tt:tt)*) => {{
        $crate::Render::render($fn($($args)*), $s);
        $crate::html2!($s; $($tt)*);
    }};
    ($s:ident; $fn:ident($($args:tt)*) { $($children:tt)* } $($tt:tt)*) => {{
        $crate::Render::render($fn($($args)*, $crate::html!($($children)*)), $s);
        $crate::html2!($s; $($tt)*);
    }};

    ($s:ident; $c:literal $($tt:tt)*) => {{
        $crate::Render::render($c, $s);
        $crate::html2!($s; $($tt)*);
    }};
    ($s:ident; ($c:expr) $($tt:tt)*) => {{
        $crate::Render::render($c, $s);
        $crate::html2!($s; $($tt)*);
    }};

    (@t $t:ident) => { stringify!($t) };
    (@t $t:literal) => { $t };

    ($s:ident; @i $i:ident) => {{
        $s.push_str(" id=\"");
        $s.push_str(stringify!($i));
        $s.push_str("\"");
    }};
    ($s:ident; @i $i:literal) => {{
        $s.push_str(" id=\"");
        $s.push_str($i);
        $s.push_str("\"");
    }};
    ($s:ident; @i ($i:expr)) => {{
        $s.push_str(" id=\"");
        $s.push_str(&$i as &str);
        $s.push_str("\"");
    }};

    ($s:ident; @cs) => {};
    ($s:ident; @cs $($tt:tt)*) => {{
        $s.push_str(" class=\"");
        $($crate::html2!($s; @c $tt); $s.push_str(" ");)*;
        String::pop($s);
        $s.push_str("\"");
    }};
    ($s:ident; @c $c:literal) => { $s.push_str($c); };
    ($s:ident; @c $c:ident) => { $s.push_str(stringify!($c)); };
    ($s:ident; @c ($c:expr)) => { $s.push_str(&$c as &str); };

    ($s:ident; @kv $k:tt=$v:tt $($tt:tt)*) => {{
        $s.push_str(" ");
        $crate::html2!($s; @k $k);
        $crate::html2!($s; @v $v);
        $crate::html2!($s; @kv $($tt)*);
    }};
    ($s:ident; @kv $k:tt[$ke:expr] $($tt:tt)*) => {{
        if $ke {
            $s.push_str(" ");
            $crate::html2!($s; @k $k);
        }
        $crate::html2!($s; @kv $($tt)*);
    }};
    ($s:ident; @kv) => {};

    ($s:ident; @k $k:ident) => { $s.push_str(stringify!($k)); };
    ($s:ident; @k $k:literal) => { $s.push_str($k); };

    ($s:ident; @v ()) => {};
    ($s:ident; @v $v:literal) => { $s.push_str(concat!("=", "\"", $v, "\"")); };
    ($s:ident; @v ($v:expr)) => {{
        $s.push_str("=");
        $s.push_str("\"");
        $s.push_str(&$v);
        $s.push_str("\"");
    }};

    ($s:ident;) => {}
}

#[macro_export]
macro_rules! render_id {
    ($s:ident;) => {};
    ($s:ident; $($idi:ident)? $($idl:literal)?) => {
        $s.push_str(concat!(" id=\"", $(stringify!($idi))* $($idl)*, '"'));
    };
    ($s:ident; $ide:expr) => {
        $s.push_str(" id=\"");
        $s.push_str(&$ide as &str);
        $s.push('"');
    };
}

#[macro_export]
macro_rules! render_classes {
    ($s:ident; @0) => {};
    ($s:ident; @0 $($tt:tt)+) => {
        $s.push_str(" class=\"");
        $crate::render_classes!($s; @n $($tt)*);
        $s.pop();
        $s.push('"');
    };
    ($s:ident; @n $($classi:ident)? $($classl:literal)?, $classcond:expr; $($tt:tt)*) => {
        if $classcond {
            $s.push_str(concat!($(stringify!($classi))* $($classl)*, ' '));
        }
        $crate::render_classes!($s; @n $($tt)*);
    };
    ($s:ident; @n $($classi:ident)? $($classl:literal)?,; $($tt:tt)*) => {
        $s.push_str(concat!($(stringify!($classi))* $($classl)*, ' '));
        $crate::render_classes!($s; @n $($tt)*);
    };
    ($s:ident; @n $classe:expr, $classcond:expr; $($tt:tt)*) => {
        if $classcond {
            $s.push_str(&$classe as &str);
            $s.push(' ');
        }
        $crate::render_classes!($s; @n $($tt)*);
    };
    ($s:ident; @n $classe:expr,; $($tt:tt)*) => {
        $s.push_str(&$classe as &str);
        $s.push(' ');
        $crate::render_classes!($s; @n $($tt)*);
    };
    ($s:ident; @n) => {}
}

#[macro_export]
macro_rules! render_keyvalue {
    ($s:ident; $key:ident;; $value:expr) => {
        if let Some(value) = $value {
            $s.push_str(concat!(' ', stringify!($key), "=\""));
            $s.push_str(value);
            $s.push('"');
        }
    };
    ($s:ident; $key:ident; $value:literal) => {
        $s.push_str(concat!(' ', stringify!($key), "=\"", $value, '"'));
    };
    ($s:ident; $key:ident; $value:expr) => {
        $s.push_str(concat!(' ', stringify!($key), "=\""));
        $s.push_str(&$value as &str);
        $s.push('"');
    };
    ($s:ident; $key:ident $cond:expr) => {
        if $cond {
            $s.push_str(concat!(' ', stringify!($key)));
        }
    };
    ($s:ident; $key:ident) => {
        $s.push_str(concat!(' ', stringify!($key)));
    };
}
