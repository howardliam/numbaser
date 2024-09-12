const HEX: &str = "0123456789ABCDEF";

pub fn hex_from_dec(input: u64) -> String {
    let mut string = String::new();

    let mut copy = input;

    while copy > 0 {
        let res = copy % 16;

        match HEX.chars().nth(res as usize) {
            Some(c) => string.push(c),
            None => {
                panic!("Inaccessible from hex");
            }
        }

        copy /= 16;
    }

    string.push_str("x0");
    string = string.chars().rev().collect::<String>();

    string
}
