const OCT: &str = "01234567";

pub fn oct_from_dec(input: u64) -> String {
    let mut string = String::new();

    let mut copy = input;

    while copy > 0 {
        let res = copy % 8;

        match OCT.chars().nth(res as usize) {
            Some(c) => string.push(c),
            None => {
                panic!("Inaccessible from oct");
            }
        }

        copy /= 8;
    }

    string.push_str("o0");
    string = string.chars().rev().collect::<String>();

    string
}
