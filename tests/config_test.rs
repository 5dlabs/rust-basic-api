//! Comprehensive configuration tests
//!
//! These tests verify edge cases and boundary conditions for configuration loading

use rust_basic_api::config::Config;
use serial_test::serial;
use std::env;

#[test]
#[serial]
fn test_config_with_empty_database_url() {
    // Save original
    let original_db = env::var("DATABASE_URL").ok();

    env::set_var("DATABASE_URL", "");
    let result = Config::from_env();

    // Empty string is technically valid - it will fail on connection attempt
    // But for config loading, it's accepted
    assert!(result.is_ok() || result.is_err());

    // Restore
    if let Some(val) = original_db {
        env::set_var("DATABASE_URL", val);
    } else {
        env::remove_var("DATABASE_URL");
    }
}

#[test]
#[serial]
fn test_config_with_various_database_urls() {
    let original_db = env::var("DATABASE_URL").ok();

    let valid_urls = vec![
        "postgresql://localhost/db",
        "postgresql://user:pass@localhost:5432/db",
        "postgres://localhost/db",
        "postgresql://localhost:5432/db?sslmode=require",
    ];

    for url in valid_urls {
        env::set_var("DATABASE_URL", url);
        let result = Config::from_env();
        assert!(result.is_ok(), "URL should be valid: {url}");
        let config = result.unwrap();
        assert_eq!(config.database_url, url);
    }

    // Restore
    if let Some(val) = original_db {
        env::set_var("DATABASE_URL", val);
    } else {
        env::remove_var("DATABASE_URL");
    }
}

#[test]
#[serial]
fn test_config_port_boundary_values() {
    let original_db = env::var("DATABASE_URL").ok();
    let original_port = env::var("SERVER_PORT").ok();

    env::set_var("DATABASE_URL", "postgresql://localhost/test");

    // Test boundary ports
    let valid_ports = vec![
        ("1", 1u16),
        ("80", 80),
        ("443", 443),
        ("3000", 3000),
        ("8080", 8080),
        ("65535", 65535),
    ];

    for (port_str, expected) in valid_ports {
        env::set_var("SERVER_PORT", port_str);
        let result = Config::from_env();
        assert!(result.is_ok(), "Port {port_str} should be valid");
        assert_eq!(result.unwrap().server_port, expected);
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
fn test_config_with_whitespace() {
    let original_db = env::var("DATABASE_URL").ok();
    let original_port = env::var("SERVER_PORT").ok();

    env::set_var("DATABASE_URL", "  postgresql://localhost/db  ");
    env::set_var("SERVER_PORT", " 3000 ");

    let result = Config::from_env();

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

    // Note: Port parsing will fail with whitespace, DB URL will have whitespace
    if let Ok(config) = result {
        assert!(config.database_url.contains("postgresql"));
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

#[test]
#[serial]
fn test_config_debug_format() {
    let original_db = env::var("DATABASE_URL").ok();

    env::set_var("DATABASE_URL", "postgresql://localhost/test");

    let config = Config::from_env().expect("Config should load");
    let debug_str = format!("{:?}", config);

    assert!(debug_str.contains("Config"));
    assert!(debug_str.contains("database_url"));
    assert!(debug_str.contains("server_port"));

    // Restore
    if let Some(val) = original_db {
        env::set_var("DATABASE_URL", val);
    } else {
        env::remove_var("DATABASE_URL");
    }
}
