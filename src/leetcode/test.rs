pub mod test {
    pub fn print(content: &String) {
        println!("{content}");
    }
}

pub fn test_print() {
    let s = String::from("hello");
    test::print(&s);
    println!("{s}")
}

