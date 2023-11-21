// Allow the following lints as in some cases C uses slightly different code conventions
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// The C function bindings are included through a macro here
include!("./bindings.rs");

struct RustStruct {
    s: *mut CStruct,
}

/// Wrapper/re-implementation of the C Struct and its functions  
/// Essentially this is a wrapper struct with the goal of providing a safe "API" for the C Struct for usage in Rust code without needing to use unsafe blocks for every usage
impl RustStruct {
    // wrapper/re-implementation of the new_c(i) function
    fn new(i: i32) -> Result<Self, ()> {
        // Call the `new_c(i)` function of the CStruct which returns a pointer to the struct
        // SAFETY: s could be a null pointer
        let s = unsafe { new_c(i) };
        if s.is_null() {
            // If s is a nullptr, return an Error
            return Err(());
        }
        // Return a RustStruct, which contains a valid pointer to a CStruct
        Ok(RustStruct { s })
    }

    // a wrapper implementation of a getter
    fn get_i(&self) -> i32 {
        // SAFETY: the RustStruct contructor checks if the Pointer is valid during creation of the object → no manual UB handling required
        unsafe { (*self.s).i }
    }
}

/// Implements the Drop trait for the RustStruct  
/// This can be considered as an override of the default drop function.
impl Drop for RustStruct {
    // This tells rust to call the drop_c function of the CStruct whenever drop is called on the RustStruct
    fn drop(&mut self) {
        // SAFETY: the drop function should function properly (responsibility of the C program)
        unsafe { drop_c(self.s) };
    }
}

fn main() -> Result<(), ()> {
    let my_struct = RustStruct::new(1)?; // implicit error handling with the ? Operator (this is a lazy way of doing error handling)
    println!("my_struct->i = {}", my_struct.get_i());
    Ok(())
} // my_struct falls out of scope → drop is called implicitly → drop_c is called → there is no memory leak
