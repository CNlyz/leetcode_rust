pub enum Gender {
    Male,
    Female
}

pub trait Info {
    fn get_name(&self) -> &str;
    fn get_age(&self) -> u8;
}

pub trait Behavior {
    fn is_friend(&self, name: &String) -> bool;
    fn add_friends(&mut self, new_friends: &Vec<String>);
}

pub struct Person<'a> {
    name: &'a str,
    age: u8,
    height: f32,
    gender: Gender,
    friends: Vec<String>
}

impl Person<'a> {
    pub fn new(name: &'a str, age: u8, height: f32, gender: Gender) -> Person<'a> {
        Person {
            name,
            age,
            height,
            gender,
            friends: vec![]
        }
    }
}

impl Info for Person {
    fn get_name(&self) -> &str {
        return &self.name;
    }

    fn get_age(&self) -> u8 {
        return self.age;
    }
}

impl Behavior for Person {
    fn is_friend(&self, name: &String) -> bool {
        let result = self.friends.contains(name);
        let mut v = vec![];
        return result;
    }

    fn add_friends(&mut self, new_friends: &Vec<String>) {
        let friends = [&self.friends, new_friends].concat();
        self.friends = friends;
    }
}