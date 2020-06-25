use assert_cmd::prelude::*;
use std::env;
use std::process::Command;
use wrangler::fixtures::{Fixture, KvConfig, WranglerToml};

#[test]
fn can_create_preview_namespace() {
    let fixture = Fixture::new();
    let mut wrangler_toml = WranglerToml::javascript("test-preview-javascript");
    wrangler_toml.kv_namespaces = Some(vec![KvConfig {
        binding: Some("BINDING"),
        id: Some("1234"),
    }]);
    fixture.create_wrangler_toml(wrangler_toml);
    preview_namespace_creation_succeeds(&fixture);
}

fn preview_namespace_creation_succeeds(fixture: &Fixture) {
    env::remove_var("CF_ACCOUNT_ID");
    env::remove_var("CF_ZONE_ID");
    let mut create_namespace = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    create_namespace.current_dir(fixture.get_path());
    create_namespace
        .arg("kv:namespace")
        .arg("create")
        .arg("BINDING")
        .arg("--preview");
    create_namespace.assert().success();
}
