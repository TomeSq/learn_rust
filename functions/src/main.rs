fn main() {
    println!("Hello, world!");

    another_function();
    another_function_param(5);
    print_labeled_measurement(5, 'h');

    includes_sentences_and_expressions();

    let five = five();
    println!("The value of five is: {}", five);

    let plus = plus_one(5);
    println!("The value of plus is: {}", plus);
}

fn another_function() {
    println!("Another function");
}

fn another_function_param(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is : {}{}", value, unit_label);
}

fn includes_sentences_and_expressions() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
