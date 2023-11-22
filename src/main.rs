// include the bindings to our C library from the bindings.rs file
mod bindings;

// This is the Rust version of a "using namespace" declaration
// it enables the use of e.g. MyCStruct directly without specifying the module where it lives.
use crate::bindings::*;

/// Declaration of a wrapper-struct, which will be used to interact with the MyCStruct struct
/// The definition/implementation of this struct and its functions are done via the following impl-s
struct MyStructWrapper {
    pub s: *mut MyCStruct,
}

/// Essentially this is a wrapper struct with the goal of providing a safe "API" for the C Struct
/// It's there to enable the usage of the MyCStruct in Rust code without needing to use unsafe blocks for every usage
impl MyStructWrapper {
    /// wrapper/re-implementation of the `new(int x)` function
    fn new(x: i32) -> Result<Self, ()> {
        // Call the `new(…)` function of the MyCStruct which returns a pointer to the struct
        // SAFETY: s could be a null pointer → error handling required!
        let s = unsafe { new(x) };

        // If s is a nullptr, return an Error
        if s.is_null() {
            return Err(());
        }

        // Return a MyStructWrapper, which contains a valid pointer to a MyCStruct
        Ok(MyStructWrapper { s })
    }

    /// implementation of a safe getter for x
    fn get_x(self: &Self) -> Option<i32> {
        // get the raw pointer to the MyCStruct inside our wrapper struct
        let s = self.s;

        // check if s is a valid pointer
        if !s.is_null() {
            // get the x value of the MyCStruct by dereferencing the raw pointer
            // SAFETY: since we have a valid pointer we can assume that the x value of the struct is also valid. The only way we initialize MyCStruct is through the new(…) function which guarantees that x is set.
            let x = unsafe { (*s).x };

            // return the value of x
            Some(x)
        } else {
            // if s is invalid return a None value instead of x
            None
        }
    }

    /// wrapper/re-implementation of the `multiply(MyCStruct *s, int factor)` function
    fn multiply(self: &Self, factor: i32) {
        // get the raw pointer to the MyCStruct inside our wrapper struct
        let s = self.s;

        // check if s is a valid pointer
        if !s.is_null() {
            // SAFETY: It's not the wrapper's responsibility to make sure the C multiply function works correctly → no extra error handling needed
            unsafe { multiply(self.s, factor) }
        }
    }
}

/// Implements the Drop trait for the MyStructWrapper  
/// This can be considered as an override of the default drop function.
impl Drop for MyStructWrapper {
    // This tells rust to call the drop_c function of the CStruct whenever drop is called on the MyStructWrapper
    fn drop(&mut self) {
        // SAFETY: the drop function should function properly (responsibility of the C program)
        unsafe { drop(self.s) };
    }
}

/// The main function of our program
fn main() -> Result<(), ()> {
    // create a wrapper struct containing a MyCStruct initialized with 2
    let my_struct = MyStructWrapper::new(2)?; // implicit error handling with the ? Operator (this is a lazy way of doing error handling)

    // Try to get x of our Struct, if successful: print x
    if let Some(x) = my_struct.get_x() {
        println!("my_struct->x = {}", x);
    }

    // multiply the x value of MyStruct by 3
    my_struct.multiply(3);

    // Try to get x of our Struct, if successful: print x
    if let Some(x) = my_struct.get_x() {
        println!("my_struct->x = {}", x);
    }

    // Instead of using our wrapper struct it's also possible to use the MyCStruct directly.
    // ⚠ However: This requires the use of unsafe
    // SAFETY: There isn't any…
    // unsafe {
    //     let mut s = Box::new(MyCStruct { x: 3 });
    //     println!("my_struct->x = {}", s.x);
    //     multiply(&mut *s, 2);
    //     println!("my_struct->x = {}", s.x);
    // }

    Ok(())
} // my_struct falls out of scope → drop() (MyStructWrapper) is called implicitly → drop(…) (MyCStruct) is called → there is no memory leak
