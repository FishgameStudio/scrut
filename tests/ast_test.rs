use syn::Item;
use syn::parse_str;

#[test]
fn test_ast() {
    let code = r#"fn main() { println!("Hello, world!"); }"#;
    let ast: Item = parse_str(code).unwrap();
    assert!(
        matches!(ast, Item::Fn(_)),
        "parsed item should be a function"
    );

    if let Item::Fn(func) = ast {
        assert_eq!(func.sig.ident.to_string(), "main");
    }
}
