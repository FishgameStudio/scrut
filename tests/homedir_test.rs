use dirs::home_dir;

#[test]
fn get_home_dir() {
    let home = home_dir();
    assert!(home.is_some());
}
