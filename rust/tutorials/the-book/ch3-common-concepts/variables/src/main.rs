fn main() {
    // println!("Hello, world!");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces: {}", spaces);

    //3.3
    another_fun(13);

    println!(
        "こんな形でも{{}}の最後の値が帰ってきます: {}",
        {
            let x = 3;
            x + 1
        }
    );

    //3.5
    let a = [123, 234, 345, 456, 567];
    for v in a.iter() {
        println!("ループしているぞい: {}", v);
    }
}

fn another_fun(x: i32) {
    println!(
        "私はmainではないところから呼ばれています。渡されたのは{}",
        x
    );
}
