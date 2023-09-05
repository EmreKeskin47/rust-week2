//Ödev1
pub fn hello(name: &str) -> String {
    format!("Hello {}!", name)
}

//Ödev2
pub fn make_double(num: i32) -> i32 {
    num * 2
}

//Ödev3
pub fn multiply_pi(num: f32) -> f32 {
    // const PI: f32 = 3.141_592_7;
    use std::f32::consts::PI;
    num * PI
}
