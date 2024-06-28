use chrono::prelude::*;

fn print_today() {
    let full_weekday: [&str; 7] = ["Sunday", "Monday", "Tuesday", "Wednesday",
    "Thursday", "Friday", "Saturday"];
    let now: DateTime<Local> = Local::now();
    println!("{}", now.weekday().num_days_from_sunday());
    // let weekday = &full_weekday[now.weekday().num_days_from_sunday()];

}

fn main() {
    let local: DateTime<Local> = Local::now();
    // println!("Today is {}", local.weekday());
    // println!("{}", local.weekday().num_days_from_sunday());
    print_today();
}
