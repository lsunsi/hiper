/// takes a template definition and returns a render implementation

#[macro_export]
macro_rules! html {
    ($($tt:tt)*) => { move |s: &mut String| $crate::render!(s; $($tt)*) }
}

#[doc(hidden)]
#[macro_export]
macro_rules! render {
    ($s:ident;
        $($tag:ident)-+
        $(# $($idi:ident)-+)?
        $(# $idl:literal)?
        $(# ($ide:expr))?
        $(. $($classi:ident)-+ $([$classicond:expr])?)*
        $(. $classl:literal $([$classlcond:expr])?)*
        $(. ($classe:expr) $([$classecond:expr])?)*
        $($($key:ident)-+ $([$keycond:expr])? $(= $($valuel:literal)? $(($valuee:expr))? $([$valuecond:expr])?)?)*
        ;
        $($sibling:tt)*
    ) => {{
        $s.push_str(concat!('<', $crate::kebabident!($($tag)*)));
        $crate::render_classes!($s; @0 $($($classi)*, $($classicond)*;)* $($classl, $($classlcond)*;)* $($classe, $($classecond)*;)*);
        $crate::render_id!($s; $($($idi)*)* $($idl)* $($ide)*);
        $($crate::render_keyvalue!($s; $($key)*; $($keycond)*$(; $($valuel)* $($valuee)* $(; $valuecond)*)*);)*
        $s.push('>');
        $crate::render!($s; $($sibling)*);
    }};
    ($s:ident;
        $($tag:ident)-+
        $(# $($idi:ident)-+)?
        $(# $idl:literal)?
        $(# ($ide:expr))?
        $(. $($classi:ident)-+ $([$classicond:expr])?)*
        $(. $classl:literal $([$classlcond:expr])?)*
        $(. ($classe:expr) $([$classecond:expr])?)*
        $($($key:ident)-+ $([$keycond:expr])? $(= $($valuel:literal)? $(($valuee:expr))? $([$valuecond:expr])?)?)*
        { $($child:tt)* }
        $($sibling:tt)*
    ) => {{
        $s.push_str(concat!('<', $crate::kebabident!($($tag)*)));
        $crate::render_classes!($s; @0 $($($classi)*, $($classicond)*;)* $($classl, $($classlcond)*;)* $($classe, $($classecond)*;)*);
        $crate::render_id!($s; $($($idi)*)* $($idl)* $($ide)*);
        $($crate::render_keyvalue!($s; $($key)*; $($keycond)*$(; $($valuel)* $($valuee)* $(; $valuecond)*)*);)*
        $s.push('>');
        $crate::render!($s; $($child)*);
        $s.push_str(concat!("</", $crate::kebabident!($($tag)*), '>'));
        $crate::render!($s; $($sibling)*);
    }};

    ($s:ident; let $i:ident = $e:expr; $($tt:tt)*) => {{
        let $i = $e;
        $crate::render!($s; $($tt)*);
    }};

    ($s:ident; match ($e:expr) { $($p:pat => {$($b:tt)*}),+ } $($tt:tt)*) => {{
        match $e { $($p => $crate::render!($s; $($b)*),)+ }
        $crate::render!($s; $($tt)*);
    }};

    ($s:ident; if let $cond:pat = $target:ident { $($itt:tt)* } else { $($ett:tt)* } $($tt:tt)*) => {{
        if let $cond = $target { $crate::render!($s; $($itt)*) } else { $crate::render!($s; $($ett)*) }
        $crate::render!($s; $($tt)*);
    }};
    ($s:ident; if let $cond:pat = $target:ident { $($itt:tt)* } $($tt:tt)*) => {{
        if let $cond = $target { $crate::render!($s; $($itt)*) }
        $crate::render!($s; $($tt)*);
    }};

    ($s:ident; if ($icond:expr) { $($ifbody:tt)* } $(else if ($eicond:expr) { $($eibody:tt)* })+ else { $($ebody:tt)* } $($tt:tt)*) => {{
        (if $icond { $crate::render!($s; $($ifbody)*) }
        $(else if $eicond { $crate::render!($s; $($eibody)*) })+
        else { $crate::render!($s; $($ebody)*) });
        $crate::render!($s; $($tt)*);
    }};
    ($s:ident; if ($icond:expr) { $($ifbody:tt)* } $(else if ($eicond:expr) { $($eibody:tt)* })+; $($tt:tt)*) => {{
        (if $icond { $crate::render!($s; $($ifbody)*); }
        $(else if $eicond { $crate::render!($s; $($eibody)*); })+);
        $crate::render!($s; $($tt)*);
    }};
    ($s:ident; if ($icond:expr) { $($ibody:tt)* } else { $($ebody:tt)* } $($tt:tt)*) => {{
        (if $icond { $crate::render!($s; $($ibody)*) }
        else { $crate::render!($s; $($ebody)*) });
        $crate::render!($s; $($tt)*);
    }};
    ($s:ident; if ($icond:expr) { $($ibody:tt)* } $($tt:tt)*) => {{
        (if $icond { $crate::render!($s; $($ibody)*); });
        $crate::render!($s; $($tt)*);
    }};

    ($s:ident; for ($p:pat in $e:expr) { $($body:tt)* } $($tt:tt)*) => {{
        for $p in $e { $crate::render!($s; $($body)*); }
        $crate::render!($s; $($tt)*);
    }};

    ($s:ident; $fn:ident($($args:tt)*); $($tt:tt)*) => {{
        $crate::Render::render($fn($($args)*), $s);
        $crate::render!($s; $($tt)*);
    }};
    ($s:ident; $fn:ident($($args:tt)*) { $($children:tt)* } $($tt:tt)*) => {{
        $crate::Render::render($fn($($args,)* $crate::html!($($children)*)), $s);
        $crate::render!($s; $($tt)*);
    }};

    ($s:ident; $c:literal $($tt:tt)*) => {{
        $crate::Render::render($c, $s);
        $crate::render!($s; $($tt)*);
    }};
    ($s:ident; ($c:expr) $($tt:tt)*) => {{
        $crate::Render::render($c, $s);
        $crate::render!($s; $($tt)*);
    }};

    ($s:ident;) => {}
}

#[doc(hidden)]
#[macro_export]
macro_rules! render_id {
    ($s:ident;) => {};
    ($s:ident; $($idi:ident)* $($idl:literal)?) => {
        $s.push_str(concat!(" id=\"", $crate::kebabident!($($idi)*), $($idl,)* '"'));
    };
    ($s:ident; $ide:expr) => {
        $s.push_str(" id=\"");
        $s.push_str(&$ide as &str);
        $s.push('"');
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! render_classes {
    ($s:ident; @0) => {};
    ($s:ident; @0 $($tt:tt)+) => {
        const CLASS: &str = " class=\"";
        $s.push_str(CLASS);
        $crate::render_classes!($s; @n $($tt)*);
        if $s.ends_with(CLASS) {
            $s.truncate($s.len() - CLASS.len());
        } else {
            $s.pop();
            $s.push('"');
        }
    };
    ($s:ident; @n $($classi:ident)* $($classl:literal)?, $classcond:expr; $($tt:tt)*) => {
        if $classcond {
            $s.push_str(concat!($(stringify!($classi))* $($classl)*, ' '));
        }
        $crate::render_classes!($s; @n $($tt)*);
    };
    ($s:ident; @n $($classi:ident)* $($classl:literal)?,; $($tt:tt)*) => {
        $s.push_str(concat!($crate::kebabident!($($classi)*), $($classl,)* ' '));
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

#[doc(hidden)]
#[macro_export]
macro_rules! render_keyvalue {
    ($s:ident; $($key:ident)+;;; $value:expr) => {
        if let Some(value) = $value {
            $s.push_str(concat!(' ', $crate::kebabident!($($key)*), "=\""));
            $s.push_str(value);
            $s.push('"');
        }
    };
    ($s:ident; $($key:ident)+;; $value:literal) => {
        $s.push_str(concat!(' ', $crate::kebabident!($($key)*), "=\"", $value, '"'));
    };
    ($s:ident; $($key:ident)+;; $value:expr) => {
        $s.push_str(concat!(' ', $crate::kebabident!($($key)*), "=\""));
        $s.push_str(&$value as &str);
        $s.push('"');
    };
    ($s:ident; $($key:ident)+; $cond:expr) => {
        if $cond {
            $s.push_str(concat!(' ', $crate::kebabident!($($key)*)));
        }
    };
    ($s:ident; $($key:ident)+;) => {
        $s.push_str(concat!(' ', $crate::kebabident!($($key)*)));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! kebabident {
    () => { "" };
    ($head:ident) => { stringify!($head) };
    ($head:ident $($tail:ident)*) => { concat!(stringify!($head), '-', $crate::kebabident!($($tail)*)) };
}
