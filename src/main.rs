mod scanner;

fn main() {
    let mut s = scanner::Scanner::new(String::from("("));
   println!("{:?}", &s.scan_tokens());
   
}
