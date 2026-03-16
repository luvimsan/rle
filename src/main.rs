use std::env;
use std::fs;
use std::process::exit;

fn help(args: Vec<String>) {
    println!("Usage: {} [encode|decode] file", args[0]);
    exit(-1);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        help(args);
    }
    else {
        let file_path = &args[2];
        let contents = fs::read_to_string(file_path).expect("Couldn't read the file");

        if args[1] == "encode" {
            let encoded = rle_encoder(contents);
            println!("{encoded}");
        }
        else if args[1] == "decode" {
            let decoded = rle_decoder(contents);
            println!("{decoded}");
        }
        else {
            help(args);
        }
    }

}

fn rle_encoder(contents: String) -> String {
    let mut chars = contents.chars();
    let mut encoded_value = String::new();

    let mut prev = chars.next().unwrap();
    let mut count: i32 = 1;

    loop {
        match chars.next() {
            Some(curr) => {
                if prev == curr {
                    count+=1;
                }
                else {
                    encoded_value.push(prev);
                    encoded_value.push_str(&count.to_string());
                    count = 1;
                }
                prev = curr;
            }
            None => break,
        }
    }
    return encoded_value;
}

fn rle_decoder(contents: String) -> String {
    let mut chars = contents.chars().peekable();
    let mut decoded_value = String::new();
    let mut len = contents.len() / 2;

    while len > 0 {
        let letter = chars.next();
        let mut count = chars.next()
                             .unwrap()
                             .to_digit(10)
                             .unwrap();

        while count > 0 {
            decoded_value.push(letter.unwrap());
            count -= 1;
        }

        len -= 1;
    }

    return decoded_value;
}
