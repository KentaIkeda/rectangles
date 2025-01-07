// 分かりやすくするために構造体を作成する
struct Rectangle {
    width: u32,
    height: u32
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