use std::path::PathBuf;

use anyhow::Error;
use policy_fetcher::policy::Policy;
use policy_fetcher::store::{path, Store};
#[test]
fn test_list() -> Result<(), Error> {
    let store = Store::new(store_root().as_path());

    let mut expected_policies = vec![
        Policy {
            uri: "https://internal.host.company/some/path/to/1.0.0/wasm-module.wasm".to_owned(),
            local_path: local_path(
                "https/internal.host.company/some/path/to/1.0.0/wasm-module.wasm",
            ),
        },
        Policy {
            uri: "registry://ghcr.io/some/path/to/wasm-module.wasm:1.0.0".to_owned(),
            local_path: local_path("registry/ghcr.io/some/path/to/wasm-module.wasm:1.0.0"),
        },
        Policy {
            uri: "registry://internal.host.company:5000/some/path/to/wasm-module.wasm:1.0.0"
                .to_owned(),
            local_path: local_path(
                "registry/internal.host.company:5000/some/path/to/wasm-module.wasm:1.0.0",
            ),
        },
    ];

    let mut list = store.list()?;

    expected_policies.sort_by_key(|p| p.uri.clone());
    list.sort_by_key(|p| p.uri.clone());

    assert_eq!(expected_policies, list);

    Ok(())
}

fn store_root() -> PathBuf {
    let store_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("test_data")
        .join("store");

    if cfg!(windows) {
        store_path.join("windows")
    } else {
        store_path.join("default")
    }
}

fn local_path(path: &str) -> PathBuf {
    store_root().join(path::encode_path(path))
}
