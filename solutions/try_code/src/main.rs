fn main() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // [2, 3]

    // This will pass silently
    assert_eq!(slice, &[2, 3]);

    // This will fail and SHOW the debug output
    assert_eq!(slice, &[3, 4]);
}

