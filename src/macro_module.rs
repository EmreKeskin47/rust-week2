#[macro_export]
macro_rules! dizi {
    [] => (
        Vec::new()
    );
    [$($x:expr),+ $(,)?] => (
        {
            let mut v = Vec::new();
            $( v.push($x); )*
            v
        }
    );
}
