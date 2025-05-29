fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    // Uncomment this line to break
    // assert_eq!(slice, &[2, 4]);
    assert_eq!(slice, &[2, 3]);
}
