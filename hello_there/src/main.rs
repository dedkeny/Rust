// BS programm to run through the Rust Documentation

// 1.   - Hello world

fn main() {
    /*
      1.1  - Comments, IYKYK
    */
    println!("\nHello, world!");
    println!("I'm a Rustagen\n\n");
    // 1.2  - Formatted Print
    let x = 5 + /* 90 + */ 5;
    println!("Comment Question:\n    let x = 5 + /* 90 + */ 5;");
    println!("Is 'x' 10 or 100?\n    x = {}\n\n", x);

    println!("{} was an int before being stringified into this statement\n\n", 31);

    println!("{0} is before {1} because {1}st place doesn't matter\n\n", 0, 1);

    println!("The {subject} is {verb} by its' {object}\n\n",
        object = "1. VARIABLES",
        subject = "2. SENTENCE",
        verb = "3. JUDGED");

    println!("Base 10  {{}} (Decimal)     : {}",   65535); // 65535
    println!("Base 2  {{:b}}(binary)      : {:b}", 65535); // 1111111111111111
    println!("Base 8  {{:o}}(octal)       : {:o}", 65235); // 177323
    println!("Base 16 {{:x}}(hexadecimal) : {:x}", 65535); // ffff
    println!("\n");


}
