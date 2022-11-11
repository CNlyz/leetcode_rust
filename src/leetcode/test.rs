pub mod test {
    pub fn test_print(content: &String) {
        println!("{content}");
    }
}

pub fn test_print1() {
    let s = String::from("hello");
    test::test_print(&s);
    println!("{s}")
}

