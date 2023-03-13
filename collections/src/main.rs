fn main() {
    // Type declaration required when not inserting values
    let _v: Vec<i32> = Vec::new();

    // Use the vec! macro to create a vector with initial values
    let _v = vec![1, 2, 3];

    // We must declare the vector as mutable if we want to change the contents
    // Here the type of v is inferred at compile time because we push i32 values to the vector
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    println!("The third element is {third}");
}
