#[cfg(test)]
mod tests {
    extern crate numeral_converter;
    use core::num;

    use numeral_converter::numeral_converter::arabic_to_roman;

    #[test]
    fn arabic_to_roman_1_I() {
        let result = arabic_to_roman(&1);
        assert_eq!(result, "I");
    }
    #[test]
    fn arabic_to_roman_2_II() {
        let result = arabic_to_roman(&2);
        assert_eq!(result, "II");
    }
    #[test]
    fn arabic_to_roman_3_III() {
        let result = arabic_to_roman(&3);
        assert_eq!(result, "III");
    }

    #[test]
    fn arabic_to_roman_5_V() {
        let result = arabic_to_roman(&5);
        assert_eq!(result, "V");
    }
    #[test]
    fn arabic_to_roman_6_VI() {
        let result = arabic_to_roman(&6);
        assert_eq!(result, "VI");
    }

    #[test]
    fn arabic_to_roman_10_X() {
        let result = arabic_to_roman(&10);
        assert_eq!(result, "X");
    }
    #[test]
    fn arabic_to_roman_11_XI() {
        let result = arabic_to_roman(&11);
        assert_eq!(result, "XI");
    }

    #[test]
    fn arabic_to_roman_15_XV() {
        let result = arabic_to_roman(&15);
        assert_eq!(result, "XV");
    }

    #[test]
    fn arabic_to_roman_18_XVIII() {
        let result = arabic_to_roman(&18);
        assert_eq!(result, "XVIII");
    }

    #[test]
    fn arabic_to_roman_4_IV() {
        let result = arabic_to_roman(&4);
        assert_eq!(result, "IV");
    }
    #[test]
    fn arabic_to_roman_9_IX() {
        let result = arabic_to_roman(&9);
        assert_eq!(result, "IX");
    }

    #[test]
    fn arabic_to_roman_19_XIX() {
        let result = arabic_to_roman(&19);
        assert_eq!(result, "XIX");
    }

    #[test]
    fn arabic_to_roman_29_XXIX() {
        let result = arabic_to_roman(&29);
        assert_eq!(result, "XXIX");
    }

}
