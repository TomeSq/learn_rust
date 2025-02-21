fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        //長方形の面積は、{}平方ピクセルです
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let react1 = (30, 50);
    println!(
        //長方形の面積は、{}平方ピクセルです
        "The area of the rectangle tuple is {} square pixels.",
        area_tuple(react1)
    );

    let react2 = Rectangle { width: 10, height: 40 };
    println!(
        //長方形の面積は、{}平方ピクセルです
        "The area of the rectangle struct is {} square pixels.",
        area_struct(&react2)
    );
    println!("rect2 is {:#?}", react2);

    //構造体のメソッドを使って面積を計算する
    println!(
        //長方形の面積は、{}平方ピクセルです
        "The area of the rectangle struct to function is {} square pixels.",
        react2.area()
    );

    //rect1にrect2がはまり込むか
    let react0 = Rectangle { width: 30, height: 50 };
    let react3 = Rectangle { width: 60, height: 45 };
    println!("Can rect1 hold rect2? {}", react0.can_hold(&react2));
    println!("Can rect1 hold rect3? {}", react0.can_hold(&react3));
}

//長方形の面積を計算する関数
//引数に関係はあるが、どこにも明記されていない
fn area(width: u32, height: u32) -> u32 {
    width * height
}

// タプルでリファクタリングする
fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

//構造体でより意味付けする
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
