use std::cell::Cell;
use std::rc::Rc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use floem_reactive::{create_effect, create_rw_signal, runtime_debug};

fn main() {
    let name = create_rw_signal("John");
    let age = create_rw_signal(20);

    let count = Rc::new(Cell::new(0));

    println!("{}", runtime_debug());
    create_effect({
        let count = count.clone();
        move |_| {
            println!("{}", runtime_debug());
            println!("inner2: age={}", age.get());
            println!("{}", runtime_debug());
            count.set(count.get() + 1);
        }
    });

    create_effect({
        let count = count.clone();
        move |_| {
            println!("inner");
            println!("{}", runtime_debug());
            println!("inner2: age={}", age.get());
            age.set(33);
            count.set(count.get() + 1);
            println!("{}", runtime_debug());
        }
    });

    println!("{}", runtime_debug());    // The effect runs once immediately

    // Setting each signal once will trigger the effect
    // println!("name.set prev");
    // name.set("Mary");
    // println!("name.set end");
    // assert_eq!(count.get(), 4);
    // println!("{}", runtime_debug());
    // let local_age = age.clone();
    // thread::spawn(move || {
    //     println!("age.set prev: age.is_none={}", local_age.try_get_untracked().is_none());
    //     local_age.set(21);
    //     println!("age.set end");
    // });
    // sleep(Duration::from_secs(1));
    // println!("assert_eq: {}", age.get_untracked());
    // println!("{}", runtime_debug());    assert_eq!(count.get(), 4);
    //
    //
    // // Setting each signal once will trigger the effect
    // println!("name.get prev");
    // name.get();
    // println!("name.get end");
    // println!("{}", runtime_debug());    assert_eq!(count.get(), 4);
}
