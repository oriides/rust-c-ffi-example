#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("./bindings.rs");
/* bindings zu den C Funktionen weggelassen um Platz zu sparen */
struct RustStruct {
    s: *mut CStruct,
}
impl RustStruct {
    fn new(i: i32) -> Result<Self, ()> {
        // SAFETY: s könnte ein null pointer sein
        let s = unsafe { new_c(i) };
        if s.is_null() {
            // falls s ein null pointer ist -> Error!
            return Err(());
        }
        // Rückgabe eines RustStructs, der einen
        // gültigen Pointer zu einem CStruct hält
        Ok(RustStruct { s })
    }
    fn get_i(&self) -> i32 {
        // SAFETY: der Konstruktor des RustStructs prüft bei Erstellung
        // ob der Pointer gültig ist -> hier kann kein UB entstehen.
        unsafe { (*self.s).i }
    }
}
impl Drop for RustStruct {
    fn drop(&mut self) {
        // SAFETY: die drop Funktion sollte nicht fehlschlagen
        unsafe { drop_c(self.s) };
    }
}
fn main() -> Result<(), ()> {
    let my_struct = RustStruct::new(1)?; // implizites Error handling mit ? Operator
    println!("my_struct->i = {}", my_struct.get_i());
    Ok(())
} // my_struct fällt aus dem scope -> drop wird implizit aufgerufen,
  // es entsteht kein memory leak
