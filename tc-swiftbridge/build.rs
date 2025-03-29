use std::path::PathBuf;
use std::env;

fn main() {
    let build_bridge = env::var("BUILD_SWIFT").map(|v| v == "1").unwrap_or(false);

    if build_bridge {
        let out_dir = PathBuf::from("./generated");

        let bridges = vec!["src/lib.rs", "src/task.rs"];
        for path in &bridges {
            println!("cargo:rerun-if-changed={}", path);
        }

        swift_bridge_build::parse_bridges(bridges)
            .write_all_concatenated(out_dir, env!("CARGO_PKG_NAME"));
    }
}
