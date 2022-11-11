pub mod leetcode;
// pub mod learn_traits;
// use learn_traits::{ Gender, Person };

fn main() {
    // let p = Person::new(String::from("Mike"), 30, 180.0, Gender::Male);
    // let friends = String::from("Tom");
    // let mut new_friends = Vec::new();
    // new_friends.push(friends);
    // p.add_friends(new_friends);
    // let string1 = String::from("abcd");
    // let string2 = "xyz";

    // let result = longest(string1.as_str(), string2);
    // println!("The longest string is {}", result);
    let s: u8 = 12;
    let s1 = s;
    println!("{s1}");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}