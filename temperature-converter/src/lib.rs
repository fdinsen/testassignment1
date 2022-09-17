mod temperature_converter{
    pub fn f_to_c(fahrenheit: f32) -> f32 {
        return (fahrenheit - 32.0) * (5.0/9.0);
    }

    pub fn c_to_f(celsius: f32) -> f32 {
        return (celsius * (9.0/5.0)) + 32.0; 
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;
    use crate::temperature_converter::{f_to_c, c_to_f};

    #[test]
    fn ftoc_50_10() {
        let result = f_to_c(50.0);
        assert!( approx_eq!(f32, result, 10.0, epsilon = 0.001));
    }

    #[test]
    fn ftoc_100_3777(){
        let result = f_to_c(100.0);
        assert!(approx_eq!(f32, result, 37.777, epsilon = 0.001));
    }

    #[test]
    fn ftoc_6point9_neg13point94() {
        let result = f_to_c(6.9);
        assert!(approx_eq!(f32, result, -13.944, epsilon = 0.001));
    }
    
    #[test]
    fn ftoc_neg40_neg40() {
        let result = f_to_c(-40.0);
        assert!(approx_eq!(f32, result, -40.0, epsilon = 0.001));
    }

    #[test]
    fn ctof_420_788(){
        let result = c_to_f(420.0);
        assert!( approx_eq!(f32, result, 788.0, epsilon = 0.001));
    }

    #[test]
    fn ctof_52_125point6() {
        let result = c_to_f(52.0);
        assert!( approx_eq!(f32, result, 125.6, epsilon = 0.001));
    }
    #[test]
    fn ctof_neg42_neg43point6() {
        let result = c_to_f(-42.0);
        assert!( approx_eq!(f32, result, -43.6, epsilon = 0.001));
    }
}
