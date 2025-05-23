fn main() {
    let c = 'z';
    println!("An implicit char: {c}");

    // See https://en.wikipedia.org/wiki/List_of_Unicode_characters
    let z: char = '\u{2124}';
    println!("Blackboard bold Z: {z}");

    // See https://www.compart.com/en/unicode/U+1F63B
    let heart_eyed_cat = '\u{1f63b}';
    println!("A heart-eyed cat {heart_eyed_cat}");
}
