# Asteroids

a set of small macros

## conds

```rust
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
```

simple pattern matching

## vec2

```rust
#[test]
fn test_make_2d_vec() {
    assert_eq!(
        crate::vec2![
            1 2 3 4 5;
            6 7 8 9 10;
            11 12 13 14 15
        ],
        vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15]
        ]
    )
}
```

create 2d-vector
