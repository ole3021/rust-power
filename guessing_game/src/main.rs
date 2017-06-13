extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("猜到正确的数字");

    let secure_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("请输入你要猜测的数字:");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("输入内容获取错误！");

        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match input.cmp(&secure_number) {
            Ordering::Less => println!("输入数字偏小"),
            Ordering::Greater => println!("输入数字偏大"),
            Ordering::Equal => {
                println!("恭喜！猜对了正确数字.");
                break;
            }
        }
    }
}
