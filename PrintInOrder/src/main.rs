use std::sync::{Arc, Mutex, Condvar};
use std::thread;

struct Foo{
    state: Mutex<u8>,
    second_cv: Condvar,
    third_cv: Condvar,
}

impl Foo {
    pub fn new() -> Self{
        Foo{
            state: Mutex::new(1),
            second_cv: Condvar::new(),
            third_cv: Condvar::new(),
        }
    }

    pub fn first(&self){
        let mut state = self.state.lock().unwrap();
        println!("first");
        *state = 2;
        self.second_cv.notify_one();
    }

    pub fn second(&self){
        let mut state = self.state.lock().unwrap();

        while *state != 2  {
            state = self.second_cv.wait(state).unwrap()
        }

        println!("second");
        *state=3;
        self.third_cv.notify_one();

    }
    pub fn third(&self){
        let mut state = self.state.lock().unwrap();

        while *state != 3  {
            state = self.third_cv.wait(state).unwrap()
        }

        println!("third");
    }
}

fn main() {
    let foo = Arc::new(Foo::new());
    let foo1 = Arc::clone(&foo);
    let foo2=Arc::clone(&foo);

    let handl_a = thread::spawn(move || {
        foo.first();
    });

    let handl_b = thread::spawn(move || {
        foo1.second();
    });

    let handl_c = thread::spawn(move || {
        foo2.third();
    });


    handl_a.join().unwrap();
    handl_b.join().unwrap();
    handl_c.join().unwrap();

    println!("Hello, world!");
}
