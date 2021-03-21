#[macro_export]
macro_rules! vec2 {
    ( $( $( $x:expr )* );* ) => {
        {
            let mut v = Vec::new();
            $(
                let mut t = Vec::new();
                $(
                    t.push($x);
                )*
                v.push(t);
            )*
            v
        }
    };
}