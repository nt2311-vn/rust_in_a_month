use std::cell::RefCell;

#[derive(Debug)]
struct User {
    id: u32,
    year_registered: u32,
    username: String,
    active: RefCell<bool>,
}

fn main() {
    let user_1 = User {
        id: 1,
        year_registered: 2020,
        username: "User 1".to_string(),
        active: RefCell::new(true),
    };

    println!("{:?}", user_1.active);

    let mut borrow = user_1.active.borrow_mut();
    *borrow = false;

    let mut borrow_one = user_1.active.borrow_mut();
    let mut borrow_two = user_1.active.borrow_mut();
}
