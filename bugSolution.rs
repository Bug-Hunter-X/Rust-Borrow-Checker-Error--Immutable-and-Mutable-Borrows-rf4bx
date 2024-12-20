fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x

    *y += 1; // Modifying x through y
    println!("x = {}", x); // Output: x = 6

    // Solution: Drop the mutable reference before creating an immutable one.
    drop(y);
    let z = &x; // Now, this is fine
    println!("x = {}", *z); 
} 