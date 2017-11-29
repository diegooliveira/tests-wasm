
use std::mem;

// It's curently compiling as a binary.
fn main() {}


/// the #[no_mangle] diretive is here to keep the function name untouched
/// This function just add 55 to some value suplied
#[no_mangle] 
pub extern "C" fn sample(value: u32) -> u32 {

    value + 55
}

/// Starts a structure that is "transfered" to the javascript world
#[no_mangle]
pub extern "C" fn start() -> *mut usize {
    
    // Put the data structure in the "heap", whatever it means in the webassembly world.
    let sample = Box::new(Sample {
                              age: 25,
                              data: Vec::new(),
                          });

    // Create a pointer to the data structure and remove from the rust onwership view
    let ptr: *mut usize = unsafe { mem::transmute(sample) };
    
    // Return to the java script the pointer to the structure
    ptr
}

// Point to the heap data strucure and add 10 years.
#[no_mangle]
pub extern "C" fn add_ten_years(ptr: *mut usize) {

    // Get back the reference to the data structure
    let mut sample: Box<Sample> = unsafe { mem::transmute(ptr) };
    sample.age += 10;

    // Remove from the rust onwership view
    mem::forget(sample);
}

// Point to the heap data strucure and get the current age.
#[no_mangle]
pub extern "C" fn get_age(ptr: *mut usize) -> i32 {

    // Get back the reference to the data structure
    let sample: Box<Sample> = unsafe { mem::transmute(ptr) };

    let age = sample.age;

    // Remove from the rust onwership view
    mem::forget(sample);

    age as i32
}


/// Generate some massive amount of data to stress the memory consumpsion
#[no_mangle]
pub extern "C" fn add_data(ptr: *mut usize) {
    let mut sample: Box<Sample> = unsafe { mem::transmute(ptr) };

    for _ in 1..10000000 {
        sample.add_data();
    }

    mem::forget(sample);
}

/// Release the allocated strucutre
#[no_mangle]
pub extern "C" fn dest(ptr: *mut usize) {
    let _sample: Box<Sample> = unsafe { mem::transmute(ptr) };
}


pub struct Sample {
    age: u16,
    data: Vec<usize>,
}


impl Sample {
    fn add_data(&mut self) {
        let new_val = (self.age as usize) * self.data.len();
        self.data.push(new_val);
    }
}
