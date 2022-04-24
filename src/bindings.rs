/* Instruktion für den Rust-Compiler, die Daten des
structs so zu organisieren wie es C macht */
#[repr(C)]
struct CStruct {
    i: i32,
}

#[link(name = "cstruct")]
extern "C" {
    // Deklaration der externen C Funktionen
    fn new_c(i: i32) -> *mut CStruct;
    fn drop_c(s: *mut CStruct);
}
