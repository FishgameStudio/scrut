use syn::Item;
use syn::parse_str;

fn main() {
    let code = r#"fn main() { println!("Hello, world!"); }"#;
    let ast: Item = parse_str(code).unwrap();
    println!("{:?}", ast);
}
