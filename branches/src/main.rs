fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    bool_condition();
    nest_conditions();
    if_is_right_let();
}

fn bool_condition() {
    let number = 3;

    //bool値でないとコンパイルエラー
    //if number  {
    if number != 0 {
        println!("number was something other than 0");
    }
}

//複数if-elseのネスト
fn nest_conditions() {
    let number = 6;

    if number % 4 == 0 {
        //数値は4で割り切れます
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        //数値は3で割り切れます
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        //数値は2で割り切れます
        println!("number is divisible by 2");
    } else {
        //数値は4、3、2で割り切れません
        println!("number is not divisible by 4, 3, or 2");
    }
}

//ifはletの右辺にすることもできる
fn if_is_right_let() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    //アームの結果が異なる場合は入れられない
    //let number = if condition { 5 } else { "six" };

    //numberの値は、{}です。
    println!("The value of number is:{}", number);
}
