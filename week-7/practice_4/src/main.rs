fn main() {
    
    // Name vector
    let name = vec!["Mary","Sam","Sally","Grey","Ade","Mark","June","Ife"];

    // Age vector
    let age = vec![16,17,19,22,20,21,18,23];

    print!("\nAge allocation:\n");

    //loop to litertae elements in vector
    for i in 0..age.len()
    {
        // literating through 1 on the vector
        print!("{} is {} years old\n",name[i],age[i]);
    }
}