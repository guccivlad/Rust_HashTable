use test_project::hashtable::HashTable;

struct User {
    name: String,
    age: i32,
}

fn main() {
    let mut usres = HashTable::<i32, User>::new();

    usres.insert(1, User { name: "Alice".to_string(), age:  15});
    usres.insert(2, User { name: "Bob".to_string(), age:  15});

    for (key, value) in usres.inter() {
        println!("id: {}, name: {}, age: {}", key, (*value).name, (*value).age);
    }
}