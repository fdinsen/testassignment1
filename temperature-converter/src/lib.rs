pub mod converter {
    pub fn f_to_c(fahrenheit: f32) -> f32 {
        return (fahrenheit - 32.0) * (5.0 / 9.0);
    }

    pub fn c_to_f(celsius: f32) -> f32 {
        return (celsius * (9.0 / 5.0)) + 32.0;
    }
}