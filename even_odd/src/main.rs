fn main() {
    //Some(1)を代入すると、値は1で奇数なので[奇数です:1]と出力される。
    //Some(2)を代入すると、値は2で奇数なので[偶数です:2]と出力される。
    //Noneを代入すると、[値がありません]と出力される。
    //i32は符号付整数型を示す。
    let objective: Option<i32>=Some(5);
    match objective{
        Some(x) if x%2 == 0 => println!("偶数です:{}",x),
        Some(x) => println!("奇数です:{}",x),
        None => println!("値がありません"),
    }
}
