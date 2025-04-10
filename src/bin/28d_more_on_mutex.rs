use std::ops::AddAssign;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    // mutex is another way to change a value without using the mut keyword
    // it means mutual exclusion, and this means only one process can change it at a time

    let some_value = 5;

    let my_mutex = Mutex::new(some_value);
    let mut mutex_changer = my_mutex.lock().unwrap();

    *mutex_changer += 1;
    // inorder to have access to the value in the mutex, the lock must be lost or dropped
    // at this point, the mutex guard, which is the lock prevent any other person from
    // accessing the value in the mutex, the lock is automatically unlocked when the guard
    // goes out of scope
    println!("Mutex while locked: {:?}", my_mutex);

    drop(mutex_changer);

    println!("Mutex after lock was released: {:?}", my_mutex);
    println!("Address of Some Value: {:p}", &some_value);

    // calling .lock() on a mutex while it is already locked would cause the later lock to wait/
    // and if used incorrect would make the program to wait forever

    // println!("About to lock mutex for changer 1");
    // #[allow(unused_variables)]
    // let changer_1 = my_mutex.lock().unwrap();
    // println!("Just locked the mutex for changer 1");

    // println!("About to lock mutex for changer 2");
    // #[allow(unused_variables)]
    // let changer_2 = my_mutex.lock().unwrap();
    // println!("Just locked the mutex for changer 2");

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        let mut guard = my_mutex.lock().unwrap();
        *guard = 1000;
    });

    // println!("Mutex: {:?}", my_mutex);

    let m = Mutex::new(0);
    // if the guard is never save to a variable the lock is immediately available after the guard has been used
    for _w in 0..=10 {
        *m.lock().unwrap() += _w;
    }

    *m.lock().unwrap() = 0;
    // we did not have to drop the guard, since it was not saved in the first place
    println!("M: {:?}", m);

    // instead of blocking the current thread with the lock method, we can use the try_lock method,
    // which immediately gives back control to the control thread whethere or not the lock was acquired, d
    // it returns a result
    let has_acquired_locked = m.try_lock();

    match has_acquired_locked {
        Ok(_guard) => println!("Acquired Lock!"),
        Err(_msg) => println!("Failed to acquire lock!"),
    };

    // note to use match case or if let to handle the result, as unwrapping it would cause a panic when
    // the lock is not acquired
    println!("M: {:?}", m);
    // dbg!(m); // QUESTION: why can i not use the debugger here for the m mutex?

    // the data in the mutex can only be accessed through the guard which is gotten from lock() and try_lock()
    // which guarantees that the data is only ever accessed when the mutex is locked

    // a mutex is considered poisoned if some thread, panic while holding it data, after this by default
    // all other threads are unable to access it value since it might have been tainted

    println!("About to Acquire guard 1");
    let _guard1 = m.lock().unwrap();
    println!("Acquired guard 1");
    // println!("About to Acquire gaurd 2");
    // let _guard2 = m.lock().unwrap(); // this would hold up the program since the mutex is currently
    // locked by the first guard
    // println!("Acquired guard 2");

    let mutex = Mutex::new(0);

    let guard = mutex.lock();
    let mut data = guard.unwrap();
    data.add_assign(5); // this is equivalent of data += 5

    println!("Data: {:?}", data);
}
