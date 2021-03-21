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

