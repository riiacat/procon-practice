fn main() {
    // println!("Hello, world!");
    //4.1
    let mut s1 = String::from("hello!!");
    s1.push_str("セカイ!");

    let s2 = s1;

    //these are errors
    // s1.push_str("error");
    // println!("{}", s1);

    // s2.push_str("error");
    println!("{}", s2);

    //4.2
    println!("calc_lentgth: {}", calc_lentgth(&s2));
    println!("オリジナル文字列: {}", s2);

    let mut s_mut = s2;
    change(&mut s_mut);

    println!("可変借用を使ったfn change:{}", s_mut);

    let r = &mut s_mut;
    r.push_str("Hello mut&");
    println!("可変借用の使用: {}", r);
}

fn calc_lentgth(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", セカイ！");
}
