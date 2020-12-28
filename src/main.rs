//! 北京科贸点餐小程序 Rust 版本

extern crate rand;
use rand::Rng;
use std::env;

fn main() {
    let mut lunch_menu = Vec::new();
    let mut dinner_menu = Vec::new();
    lunch_menu.push("金谷园");
    lunch_menu.push("小吊梨汤");
    lunch_menu.push("米有理由");
    lunch_menu.push("subway");
    lunch_menu.push("肯德基");
    lunch_menu.push("海盗虾饭");
    lunch_menu.push("眉州东坡");
    lunch_menu.push("合利屋");
    lunch_menu.push("必胜客");
    lunch_menu.push("好嫂子");
    lunch_menu.push("于是乎");
    lunch_menu.push("日昌餐馆");
    lunch_menu.push("和风鳗鱼饭");
    lunch_menu.push("么哈");
    lunch_menu.push("热啊");
    lunch_menu.push("局气");
    dinner_menu.push("subway");
    dinner_menu.push("Boom 轻食");
    dinner_menu.push("田老师");
    dinner_menu.push("嘉和一品");
    dinner_menu.push("刀小蛮");
    let mut order = rand::thread_rng();
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 1usize);
    if args[1].as_str() == "lunch" {
        println!("午饭 {}", lunch_menu[order.gen_range(0..lunch_menu.len())]);
    }
    else if args[1].as_str() == "dinner" {
        println!("晚饭 {}", dinner_menu[order.gen_range(0..dinner_menu.len())]);
    }
    else {
        println!("error input: {}", args[1]);
    }
}
