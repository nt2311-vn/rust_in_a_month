use std::sync::Mutex;

fn main() {
    let my_mutex = Mutex::new(5);

    // let mut mutex_changer = my_mutex.lock().unwrap();
    // *mutex_changer = 6;
    // drop(mutex_changer);
    //
    // println!("{my_mutex:?}");
    // let mut other_mutex_changer = my_mutex.try_lock();
    //
    // if let Ok(value) = other_mutex_changer {
    //     println!("The MutexGuard has: {value}")
    // } else {
    //     println!("Didn't get the lock")
    // }
    *my_mutex.lock().unwrap() = 6;
}
