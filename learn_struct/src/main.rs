//ユーザー構造体
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    //構造体変数の作成
    let mut user1 = User {
        username: String::from("some_username123"),
        email: String::from("someone@example.com"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("another_some_email@example.com");

    let user2 = User {
        username: String::from("user02"),
        email: String::from("user02p@example.com"),
        //user1の一部の値を使用しつつ、新しいUserインスタンスを作成する。
        ..user1
    };

    //デフォルトを値を指定してUser構造体の作製
    let user3 = build_user(String::from("user3@example.com"), String::from("user3"));
}

fn build_user(email: String, username: String) -> User {
    //引数名と構造体のフィールド名が同じなら
    //フィールド初期省略記法を使って指定できる
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}
