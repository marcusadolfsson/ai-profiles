//! The server ships with the app, in the same release: they carry one version.

#[test]
fn the_server_has_the_apps_version() {
    let app = include_str!("../../Cargo.toml");
    let version = app
        .lines()
        .find_map(|line| line.strip_prefix("version = \""))
        .and_then(|rest| rest.strip_suffix('"'))
        .expect("the app's Cargo.toml has a version");
    assert_eq!(
        env!("CARGO_PKG_VERSION"),
        version,
        "set server/Cargo.toml's version to the app's"
    );
}
