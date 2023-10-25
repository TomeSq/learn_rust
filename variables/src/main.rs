use std::io;

fn main() {
    let x = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    let num = 98_222;
    println!("The value of num is: {}", num);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻'; //ハート目の猫
    let j = "日本語だよ";

    println!("The value of c: {} {} {} {}", c, z, heart_eyed_cat, j);

    //タプル
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of tup: {} {} {}", x, y, z);

    //配列型
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("The value of [0] {}", months[0]);

    // 添字エラーについて
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");
    // 配列の何番目の要素にアクセスするか指定してください

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    // 値の読み込みに失敗しました

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number"); // 入力された値は数字ではありません

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        // {}番目の要素の値は{}です
        index,
        element
    );
}
