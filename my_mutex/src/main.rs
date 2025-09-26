use std::sync::Mutex;

fn main() {
    let my_mutex = Mutex::new(5);
    let mut mutex_changer = my_mutex.lock().unwrap();

    println!("{my_mutex:?}");
    println!("{mutex_changer:?}");

    *mutex_changer = 6;
    println!("{mutex_changer:?}");
}
