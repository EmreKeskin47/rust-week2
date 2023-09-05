use rust_w2::dizi;
use rust_w2::enums_module::{divide, CustomResult};
use rust_w2::hw::{hello, make_double, multiply_pi}; // Assuming you have a module named "hw" in lib.rs
use rust_w2::traits_module::{say_your_name_and_age, Person};

fn main() {
    //ÖDEV 1 Testleri
    let greeting = hello("Alice");
    println!("{}", greeting);

    let doubled = make_double(2);
    println!("{}", doubled);

    let multiplied = multiply_pi(2.0);
    println!("{}", multiplied);

    // Custom Result Enum
    let res: CustomResult<f64, &str> = divide(10.0, 0.0);
    //let res: CustomResult<f64, &str> = divide(10.0, 2.0);

    match res {
        CustomResult::Ok(result) => println!("Sonuç: {}", result),
        CustomResult::Err(err) => println!("Hata: {}", err),
    }

    //Trait
    let s = Person {
        name: String::from("Bob"),
        age: 14,
    };

    match say_your_name_and_age(&s) {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e),
    }

    let s2 = Person {
        name: String::from("Alice"),
        age: 24,
    };

    println!("{}", s2);

    //macro
    let v = dizi![1, 2, 3];
    println!("{:?}", v);
}
