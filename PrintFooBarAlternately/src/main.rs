use std::sync::{Arc, Mutex, Condvar};
use std::thread;

struct FooBar {
    n: usize,
    is_foo_turn: Mutex<bool>,
    foo_cv: Condvar,
    bar_cv: Condvar,
}

impl FooBar {
    pub fn new(n: usize) -> Self{
        FooBar{
            n,
            is_foo_turn: Mutex::new(true),
            foo_cv: Condvar::new(),
            bar_cv: Condvar::new(),
        }
    }

    pub fn foo(&self){
        for _ in 0..self.n{
            let mut is_foo_turn = self.is_foo_turn.lock().unwrap();
            while !*is_foo_turn{
                is_foo_turn = self.foo_cv.wait(is_foo_turn).unwrap();
            }
            print!("foo");
            *is_foo_turn = false;
            self.bar_cv.notify_one();
        }
    }

    pub fn bar(&self){
        for _ in 0..self.n{
            let mut is_foo_turn = self.is_foo_turn.lock().unwrap();
            while *is_foo_turn{
                is_foo_turn = self.bar_cv.wait(is_foo_turn).unwrap();
            }
            print!("bar");
            *is_foo_turn = true;
            self.foo_cv.notify_one();
        }
    }

}

fn main() {

    let n =1000;
    let foobar = Arc::new(FooBar::new(n));
    let foobar_clone = Arc::clone(&foobar);

    let thread_foo = thread::spawn(move || {
        foobar_clone.foo();
    });

    let foobar_clone = Arc::clone(&foobar);

    let thread_bar = thread::spawn(move || {
        foobar_clone.bar();
    });

    thread_foo.join().unwrap();
    thread_bar.join().unwrap();



}
