#[test]
fn test_i32() {
    let a = crate::conds! {
        1 2 3;
        1 4 3, 9;
        3 5 2, 11;
        2 6 4, 6;
        1 2 3, 100;
    };
    assert_eq!(a.unwrap(), 100)
}

#[test]
fn test_f32() {
    let a = crate::conds! {
        1.0 2.0 3.0;
        1.0 4.0 3.0, 9.0;
        3.0 5.0 2.0, 11.0;
        2.0 6.0 4.0, 6.0;
        1.0 2.0 3.0, 100.0;
    };
    assert_eq!(a.unwrap(), 100.0)
}

#[test]
fn test_char() {
    let a = crate::conds! {
        'a' 'c' 'b';
        'b' 'a' 'd', 'a';
        'c' 'b' 'b', 'b';
        'a' 'c' 'b', 'c';
        'b' 'e' 'q', 'd';
    };
    assert_eq!(a.unwrap(), 'c')
}

#[test]
fn test_string() {
    let a = crate::conds! {
        "a" "c" "b";
        "b" "a" "d", "a";
        "c" "b" "b", "b";
        "a" "c" "b", "c";
        "b" "e" "q", "d";
    };
    assert_eq!(a.unwrap(), "c")
}

#[test]
fn test_i32_to_u64() {
    let a = crate::conds! {
        1 2 3;
        1 4 3, 102323 as u64;
        3 5 2, 234234 as u64;
        2 6 4, 452444 as u64;
        1 2 3, 123342 as u64;
    };
    assert_eq!(a.unwrap(), 123342)
}

#[test]
fn test_not_matched() {
    let a = crate::conds! {
        1 2 3;
        1 4 3, 9;
        3 5 2, 11;
        2 6 4, 6;
        1 2 8, 0;
    };
    assert_eq!(a, None)
}
