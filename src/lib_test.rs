use std::env;
use super::get_port;

#[test]
fn test_port_configuration() {
    // Test default port
    assert_eq!(get_port(), 3000);

    // Test custom port
    env::set_var("PORT", "8080");
    assert_eq!(get_port(), 8080);

    // Test invalid port falls back to default
    env::set_var("PORT", "invalid");
    assert_eq!(get_port(), 3000);
}
