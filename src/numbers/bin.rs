const BIN: &str = "01";

pub fn bin_from_dec(input: u64) -> String {
    let mut string = String::new();

    let mut copy = input;

    while copy != 0 {
        let res = copy % 2;

        match BIN.chars().nth(res as usize) {
            Some(c) => string.push(c),
            None => {
                panic!("Inaccessible from bin");
            }
        }

        copy /= 2;
    }

    string.push_str("b0");
    string = string.chars().rev().collect::<String>();

    string
}
