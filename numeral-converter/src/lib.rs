pub mod numeral_converter {
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