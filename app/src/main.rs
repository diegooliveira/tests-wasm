
use std::mem;

// It's curently compiling as a binary.
fn main() {}



#[no_mangle] // keep the function name untouched
pub extern "C" fn sample(value: u32) -> u32 {

    value + 55
}

#[no_mangle]
pub extern "C" fn start() -> *mut usize {

    println!("1");
    let sample = Box::new(Sample {
                              name: String::from("Jhon"),
                              age: 25,
                              data: Vec::new(),
                          });
    println!("2 Name{} ", sample.name);
    let ptr: *mut usize = unsafe { mem::transmute(sample) };
    println!("3 {:?} ", ptr);

    ptr
}

#[no_mangle]
pub extern "C" fn cont(ptr: *mut usize) {
    let mut sample: Box<Sample> = unsafe { mem::transmute(ptr) };
    sample.age += 10;
    println!("3 {:?} ", ptr);
    mem::forget(sample);
}

#[no_mangle]
pub extern "C" fn get_age(ptr: *mut usize) -> i32 {
    let sample: Box<Sample> = unsafe { mem::transmute(ptr) };

    let age = sample.age;
    mem::forget(sample);

    age as i32
}


#[no_mangle]
pub extern "C" fn add_data(ptr: *mut usize) {
    let mut sample: Box<Sample> = unsafe { mem::transmute(ptr) };

    for _ in 1..10000000 {
        sample.add_data();
    }

    mem::forget(sample);
}

#[no_mangle]
pub extern "C" fn dest(ptr: *mut usize) {
    let _sample: Box<Sample> = unsafe { mem::transmute(ptr) };
}



impl Drop for Sample {
    fn drop(&mut self) {
        println!("destroyed {}", self.name);
    }
}

pub struct Sample {
    name: String,
    age: u16,
    data: Vec<usize>,
}


impl Sample {
    fn add_data(&mut self) {
        let new_val = (self.age as usize) * self.data.len();
        self.data.push(new_val);
    }
}
#[test]
fn do_test() {

    let a = start();
    cont(a);
}
