//! Unit tests for configuration module

use rust_basic_api::config::Config;
use serial_test::serial;
use std::env;

#[test]
#[serial]
fn test_config_with_default_port() {
    let original_db = env::var("DATABASE_URL").ok();
    let original_port = env::var("SERVER_PORT").ok();

    env::set_var("DATABASE_URL", "postgresql://localhost/test");
    env::remove_var("SERVER_PORT");

    let config = Config::from_env().expect("Failed to load config");
    assert_eq!(config.server_port, 3000);
    assert_eq!(config.database_url, "postgresql://localhost/test");

    // Restore
    if let Some(val) = original_db {
        env::set_var("DATABASE_URL", val);
    } else {
        env::remove_var("DATABASE_URL");
    }
    if let Some(val) = original_port {
        env::set_var("SERVER_PORT", val);
    } else {
        env::remove_var("SERVER_PORT");
    }
}

#[test]
#[serial]
fn test_config_with_custom_port() {
    let original_db = env::var("DATABASE_URL").ok();
    let original_port = env::var("SERVER_PORT").ok();

    env::set_var("DATABASE_URL", "postgresql://localhost/test");
    env::set_var("SERVER_PORT", "8080");

    let config = Config::from_env().expect("Failed to load config");
    assert_eq!(config.server_port, 8080);

    // Restore
    if let Some(val) = original_db {
        env::set_var("DATABASE_URL", val);
    } else {
        env::remove_var("DATABASE_URL");
    }
    if let Some(val) = original_port {
        env::set_var("SERVER_PORT", val);
    } else {
        env::remove_var("SERVER_PORT");
    }
}

#[test]
#[serial]
fn test_config_missing_database_url() {
    let original_db = env::var("DATABASE_URL").ok();

    env::remove_var("DATABASE_URL");

    let result = Config::from_env();
    assert!(result.is_err());

    // Restore
    if let Some(val) = original_db {
        env::set_var("DATABASE_URL", val);
    }
}

#[test]
#[serial]
fn test_config_invalid_port() {
    let original_db = env::var("DATABASE_URL").ok();
    let original_port = env::var("SERVER_PORT").ok();

    env::set_var("DATABASE_URL", "postgresql://localhost/test");
    env::set_var("SERVER_PORT", "invalid");

    let result = Config::from_env();
    assert!(result.is_err());

    // Restore
    if let Some(val) = original_db {
        env::set_var("DATABASE_URL", val);
    } else {
        env::remove_var("DATABASE_URL");
    }
    if let Some(val) = original_port {
        env::set_var("SERVER_PORT", val);
    } else {
        env::remove_var("SERVER_PORT");
    }
}

#[test]
#[serial]
fn test_config_port_boundaries() {
    let original_db = env::var("DATABASE_URL").ok();
    let original_port = env::var("SERVER_PORT").ok();

    env::set_var("DATABASE_URL", "postgresql://localhost/test");

    // Test valid boundary ports
    for (port_str, expected) in &[("1", 1u16), ("65535", 65535)] {
        env::set_var("SERVER_PORT", port_str);
        let config = Config::from_env().expect("Port should be valid");
        assert_eq!(config.server_port, *expected);
    }

    // Test invalid ports (note: 0 is valid in Rust - means any available port)
    for invalid_port in &["70000", "-1", "abc"] {
        env::set_var("SERVER_PORT", invalid_port);
        let result = Config::from_env();
        assert!(result.is_err(), "Port {invalid_port} should be invalid");
    }

    // Restore
    if let Some(val) = original_db {
        env::set_var("DATABASE_URL", val);
    } else {
        env::remove_var("DATABASE_URL");
    }
    if let Some(val) = original_port {
        env::set_var("SERVER_PORT", val);
    } else {
        env::remove_var("SERVER_PORT");
    }
}

#[test]
#[serial]
fn test_config_clone() {
    let original_db = env::var("DATABASE_URL").ok();

    env::set_var("DATABASE_URL", "postgresql://localhost/test");
    env::remove_var("SERVER_PORT");

    let config = Config::from_env().expect("Config should load");
    let cloned = config.clone();

    assert_eq!(config.database_url, cloned.database_url);
    assert_eq!(config.server_port, cloned.server_port);

    // Restore
    if let Some(val) = original_db {
        env::set_var("DATABASE_URL", val);
    } else {
        env::remove_var("DATABASE_URL");
    }
}
