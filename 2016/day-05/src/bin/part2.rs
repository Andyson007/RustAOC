use crypto::digest::Digest;

fn main() {
    let mut result = [None; 8];
    let mut i = 0;
    let mut hasher = crypto::md5::Md5::new();
    while result.iter().any(|c| c.is_none()) {
        hasher.input(b"ojvtpuvg");
        hasher.input(i.to_string().as_bytes());
        let mut ret = [0; 16];
        hasher.result(&mut ret);
        if ret[0] == 0 && ret[1] == 0 && ret[2]>>4 == 0 {
          let index = ret[2]&0xf;
          if index < 8 {
            if result[index as usize].is_none(){
              result[index as usize] = Some(ret[3]>>4);
              println!("{index}");
            }
          }
        }
        hasher.reset();
        i+=1;
    }
    println!("{}", result.iter().map(|c|format!("{:x}", c.unwrap())).collect::<String>());
}
