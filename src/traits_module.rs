use std::fmt;

pub trait Speaker {
    fn say_name(&self) -> String;
    fn say_age(&self) -> Result<String, String>;
}

pub struct Person {
    pub name: String,
    pub age: u8,
}

impl Speaker for Person {
    fn say_name(&self) -> String {
        format!("My name is {}", self.name)
    }
    fn say_age(&self) -> Result<String, String> {
        if self.age < 18 {
            Err(String::from("Age can not be smaller than 18"))
        } else {
            Ok(format!("My age is {}", self.age))
        }
    }
}

pub fn say_your_name_and_age(speaker: &impl Speaker) -> Result<(), String> {
    println!("{}", speaker.say_name());
    println!("{}", speaker.say_age()?);
    Ok(())
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.name, self.age)
    }
}

// // veya generic tip kullanarak:
// fn say_your_name<T: Speaker>(speaker: &T) {
//     println!("{}", speaker.say_name());
// }

// // veya çalışma-zamanı kontrollü &dyn kullanarak:
// fn say_your_name(speaker: &dyn Speaker) {
//     println!("{}", speaker.say_name());
// }
