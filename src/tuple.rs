pub fn tuple1(){
        // Declare a tuple of three elements
    let tuple_e = ('E', 5i32, true);

    // Use tuple indexing and show the values of the elements in the tuple
    println!("Is '{}' the {}th letter of the alphabet? {}", tuple_e.0, tuple_e.1, tuple_e.2);
}

pub fn struct1(){
    // Classic struct with named fields
    struct Student { name: String, level: u8, remote: bool }
}