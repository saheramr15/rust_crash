fn concatenate_string(string1: &str, string2: &str) -> String {
    
    format!("{string1}{string2}") 
}
fn main() {
    
    let string1 = String::from("Saher ");
    let string2 = String::from("Amr");

    
    let result = concatenate_string(&string1, &string2);
    println!("String 1: {string1}");
    println!("String 2: {string2}");
    
    println!("Concatenated string: {result}");
}
