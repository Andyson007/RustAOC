use crypto::digest::Digest;

fn main() {
    let mut result = String::new();
    let mut i = 0;
    while result.len() < 8 {
        let mut hasher = crypto::md5::Md5::new();
        hasher.input(b"ojvtpuvg");
        hasher.input(i.to_string().as_bytes());
        let result_str = hasher.result_str();
        if result_str.starts_with("00000") {
            println!("{i}");
            result.push(result_str.to_string().chars().nth(5).unwrap());
        }
        i+=1;
    }
    println!("{result}");
}
