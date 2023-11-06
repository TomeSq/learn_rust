fn main() {
    let s = String::from("hello world");

    //'String'のスライスに対して機能する
    let word = first_word(&s[..]);
    println!("first word: {}", word);

    let s_literal = "hello world";

    //文字列のリテラルのスライスに対して機能する
    let word = first_word(&s_literal[..]);
    println!("first word (literal): {}", word);

    //文字列リテラルはそれ自体が既にスライスなのでそのまま渡しても機能する
    let word = first_word(s_literal);
    println!("first word (literal2): {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
