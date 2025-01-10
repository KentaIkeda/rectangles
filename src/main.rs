// 分かりやすくするために構造体を作成する
#[derive(Debug)] // 構造体をデバッグする為に必要な記述
struct Rectangle {
    width: u32,
    height: u32
}

// 構造体のメソッド定義
impl Rectangle {
    // メソッドの最初の引数は必ずselfである必要がある
    // このselfはメソッドが呼び出されている構造体のインスタンスを表している
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;
    // let rect1: (u32, u32) = (30, 50); // タプルで幅と高さを表現
    let rect2 = Rectangle { width: 30, height: 50 };

    println!(
        // 長方形の面積は、{}平方ピクセルです
        "The area of the rectangle is {} square pixels.",
        area(&rect2)
    );

    // :?を書くとDebugと呼ばれる出力生計を使いたいとコンパイラーに指示をしてくれる
    println!("{:?}", rect2);
}

// 四角形の面積を計算する関数
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// タプルを使用して表現した幅と高さを使って面積を計算する関数
// fn area(dimensions: (u32, u32)) -> u32 {
//     // 構造化はされたけど.0, .1となってどっちがどっちかわかりづらい。
//     // 可読性はいまいち低い
//     dimensions.0 * dimensions.1
// }

// 構造体を使用して面積を計算する関数
fn area(rectangle: &Rectangle) -> u32 {
    // 構造体を使用したことで、どっちが何なのか分かりやすくなった
    rectangle.width * rectangle.height
}