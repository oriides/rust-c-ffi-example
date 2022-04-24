#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("./bindings.rs");

fn main() {
    unsafe {
        // die new() Funktion könnte einen null pointer zurückgeben
        let my_struct = new_c(1); // Erstellen des structs über C Funktion
        if !my_struct.is_null() {
            // Abfrage, ob der pointer != null
            println!("my_struct->i = {}", (*my_struct).i); // Zugriff über raw pointer
        }
        drop_c(my_struct); // manuelle Freigabe des Speichers über C Funktion
    }
}
