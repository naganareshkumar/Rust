fn main() {
    println!("Hello, World!");
    let m_n = Some(43);
    if let Some(number) = m_n {
        println!("{}", number);
    } else {
        println!("error");
    }
    let age = get_age();
if let Some(num) = age {
    println!("Age: {}", num);
} else {
    println!("No age found");  // This handles the None case
}
}


// Might NOT have a value
fn get_age() -> Option<i32> {
    None  // or Some(25) - we don't know!
}

