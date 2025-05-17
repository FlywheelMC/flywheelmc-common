pub macro variadic( $macro:path ) {
    variadic_strip1!{ $macro => L, K, J, I, H, G, F, E, D, C, B, A, }
}

pub macro variadic_strip1 {
    ( $macro:path => $first:ident $( , $rest:ident )* $(,)? ) => {
        $macro! { $first $( , $rest )* , }
        variadic_strip1!{ $macro => $( $rest , )* }
    },
    ( $macro:path => $(,)? ) => {
        $macro! { }
    }
}
