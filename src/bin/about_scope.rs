#![allow(unused_imports)]

use floem_reactive::{create_effect, create_memo, create_rw_signal, Scope};

fn main() {
    test_1();
    test_2();
    test_3();
}


fn test_3() {
    let scope_1 = Scope::new();
    let scope_2 = Scope::new();
    let switch = scope_2.create_rw_signal(false);
    let value = scope_1.create_rw_signal(0);
    let memo = create_memo(move |_x| {
        switch.get()
    });
    println!("{scope_1:?}, {scope_2:?}, {:?}", Scope::current());
    create_effect(move |_| {
        memo.get();
        value.update(|x| *x += 1)
    });
    switch.update(|x| *x = true);
    assert_eq!(value.get_untracked(), 2);
    switch.update(|x| *x = true);
    assert_eq!(value.get_untracked(), 2);
    switch.update(|x| *x = false);
    assert_eq!(value.get_untracked(), 3);
    switch.update(|x| *x = false);
    assert_eq!(value.get_untracked(), 3);
}

fn test_2() {
    let scope_1 = Scope::new();
    let scope_2 = Scope::new();
    let switch = scope_2.create_rw_signal(false);
    let value = scope_1.create_rw_signal(0);

    println!("{scope_1:?}, {scope_2:?}, {:?}", Scope::current());
    create_effect(move |_| {
        if switch.get() {
            value.update(|x| *x += 1)
        }
    });
    switch.update(|x| *x = true);
    assert_eq!(value.get_untracked(), 1);
    scope_1.dispose();
    switch.update(|x| *x = true);
    assert!(value.try_get_untracked().is_none());
}

fn test_1() {
    let scope = Scope::new();
    let switch = scope.create_rw_signal(false);
    let value = scope.create_rw_signal(0);
    create_effect(move |_| {
        if switch.get() {
            value.update(|x| *x += 1)
        }
    });
    switch.update(|x| *x = true);
    assert_eq!(value.get_untracked(), 1);
}
