fn main() {
    //    ownership();
    //    return_ownership();
    reference();
    //reference_to_dangle();
}

/*
fn ownership() {
    //sがスコープに入る
    let s = String::from("hello");

    //sの値が関数にムーブされ
    takes_ownership(s);
    //ここからは有効でなはない

    //↓sは有効でないのでコンパイルエラー
    //println!("{}", s);

    let x = 5;

    //xが関数にムーブされるが、
    makes_copy(x);
    //i32はCopyなのでこの後にxを使っても大丈夫

    println!("{}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} //some_stringがスコープを抜けるので、`drop`が呼ばれ、メモリが解放される。

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} //some_integerがスコープを抜ける。何も特別なことはない

fn return_ownership() {
    //gives_ownership関数から戻り値をs1へムーブする
    let s1 = gives_ownership();

    //s2がスコープに入る
    let s2 = String::from("Hello");

    //s2はtakes_and_gives_backにムーブされ
    //戻りた値もs3にムーブされる
    let s3 = takes_and_gives_back(s2);
    //s1、s3はスコープを抜けるのでドロップされる。
    //s2は既にムーブ済みなので何も起きない
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
 */

fn reference() {
    let mut s1 = String::from("hello");

    change(&mut s1);

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", World!");
}

/* //ダングリング参照
fn reference_to_dangle() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("Hello");
    &s //String sの参照を返す
       // ここでsはスコープを抜けるのでドロップされる。
}
 */
