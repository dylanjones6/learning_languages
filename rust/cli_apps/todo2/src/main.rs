use todo2::TodoItem;
use todo2::Todo;
use std::env;

fn main() {
    // todo_item()
    //println!("{:?}", todo2::HELLO);
    // let example = String::from("Steven");
    // let thing = TodoItem {
    //     name: example
    // };
    // println!("{}", thing.name);
    // let ex2 = TodoItem::new(String::from("Ted"));
    // println!("{}", ex2.name);

    // let test1 = TodoItem::new(String::from("This"));
    // let test2 = TodoItem::new(String::from("test"));

    // let mut test_list = Todo::new(/*String::from("testing")*/);
    // test_list.push(test1);
    // test_list.push(test2);
    // test_list.pop();
    // &test_list.print_items();
    // &test_list.print();
    // println!("{}", test_list);
    // test_list.save();
    // test_list.read();
    let mut args: Vec<String> = env::args().collect();
    // for i in &args {
    //     println!("{}", i);
    // }
    if args.len() == 1 {
        args = vec![String::from("list")]
    } else {
        args = args[1..].to_vec();
    }
    println!("Args after change: ");
    for i in &args {
        println!("{i}");
    }
    match args[0].as_str() {
        "list" => todo2::list(),
        "add" => todo2::add(&args[1..].to_vec()),
        "check" => todo2::check(&args[1..].to_vec()),
        _ => panic!("Return a better error here!")
    }
    // print!("\n");
    // for i in &args {
    //     println!("{i}");
    // }


}
