fn main() {
    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Regular annotation
    let an_integer   = 5i32; // Suffix annotation

    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    // A type can also be inferred from context
    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;

    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // Error! The type of a variable can't be changed.
    // mutable = true;

    // Variables can be overwritten with shadowing.
    let mutable = true;

    /* Compound types - Array and Tuple */

    // Array signature: [T; N]
    let array: [i32; 3] = [1, 2, 3];
    let array = [1, 2, 3]; // Same as above, type annotation is optional

    //Tuple signature: (T, U, ..)
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let tuple = (500, 6.4, 1); // Same as above, type annotation is optional
    
}