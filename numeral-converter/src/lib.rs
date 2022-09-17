mod numeral_converter {
    pub fn arabic_to_roman(arabic: &i32) -> String {
        let mut remainder: i32;
        let mut roman = String::from("");
        let xs = arabic / 10;
        remainder = arabic % 10;

        let vs = remainder / 5;
        remainder = remainder % 5;
        let ends_with_9 = arabic % 10 == 9;
        for n in 0..xs {
            roman.push('X');
        }
        if ends_with_9 {
            roman.push('I');
            roman.push('X');
            return roman;
        }

        let ends_with_4 = arabic % 10 == 4;
        for _ in 0..vs {
            roman.push('V');
        }
        if !ends_with_4 {
            for _ in 0..remainder {
                roman.push('I');
            }
        } else {
            roman.push('I');
            roman.push('V');
        };

        return roman;
    }
}

#[cfg(test)]
mod tests {
    use core::num;

    use crate::numeral_converter;

    #[test]
    fn arabic_to_roman_1_I() {
        let result = numeral_converter::arabic_to_roman(&1);
        assert_eq!(result, "I");
    }
    #[test]
    fn arabic_to_roman_2_II() {
        let result = numeral_converter::arabic_to_roman(&2);
        assert_eq!(result, "II");
    }
    #[test]
    fn arabic_to_roman_3_III() {
        let result = numeral_converter::arabic_to_roman(&3);
        assert_eq!(result, "III");
    }

    #[test]
    fn arabic_to_roman_5_V() {
        let result = numeral_converter::arabic_to_roman(&5);
        assert_eq!(result, "V");
    }
    #[test]
    fn arabic_to_roman_6_VI() {
        let result = numeral_converter::arabic_to_roman(&6);
        assert_eq!(result, "VI");
    }

    #[test]
    fn arabic_to_roman_10_X() {
        let result = numeral_converter::arabic_to_roman(&10);
        assert_eq!(result, "X");
    }
    #[test]
    fn arabic_to_roman_11_XI() {
        let result = numeral_converter::arabic_to_roman(&11);
        assert_eq!(result, "XI");
    }

    #[test]
    fn arabic_to_roman_15_XV() {
        let result = numeral_converter::arabic_to_roman(&15);
        assert_eq!(result, "XV");
    }

    #[test]
    fn arabic_to_roman_18_XVIII() {
        let result = numeral_converter::arabic_to_roman(&18);
        assert_eq!(result, "XVIII");
    }

    #[test]
    fn arabic_to_roman_4_IV() {
        let result = numeral_converter::arabic_to_roman(&4);
        assert_eq!(result, "IV");
    }
    #[test]
    fn arabic_to_roman_9_IX() {
        let result = numeral_converter::arabic_to_roman(&9);
        assert_eq!(result, "IX");
    }

    #[test]
    fn arabic_to_roman_19_XIX() {
        let result = numeral_converter::arabic_to_roman(&19);
        assert_eq!(result, "XIX");
    }

    #[test]
    fn arabic_to_roman_29_XXIX() {
        let result = numeral_converter::arabic_to_roman(&29);
        assert_eq!(result, "XXIX");
    }

}
