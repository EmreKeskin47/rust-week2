pub enum CustomResult<T, E> {
    Ok(T),
    Err(E),
}

pub fn divide(a: f64, b: f64) -> CustomResult<f64, &'static str> {
    if b == 0.0 {
        return CustomResult::Err("Bir sayıyı sıfıra bölemezsiniz!");
    }
    CustomResult::Ok(a / b)
}
