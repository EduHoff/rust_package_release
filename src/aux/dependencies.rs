#[derive(Hash, Eq, PartialEq)]
pub struct Dependency {
    pub name: &'static str,
    pub args: &'static [&'static str],
    pub label: &'static str,
}

pub const DEPENDENCIES: &[Dependency] = &[
    Dependency {
        name: "docker",
        args: &["info"],
        label: "docker",
    },
    Dependency {
        name: "rustc",
        args: &["--version"],
        label: "rustc",
    },
    Dependency {
        name: "cargo",
        args: &["--version"],
        label: "cargo",
    },
    Dependency {
        name: "cargo-cross",
        args: &["--version"],
        label: "cargo-cross",
    },
    Dependency {
        name: "rustc",
        args: &["--target", "x86_64-apple-darwin", "--version"],
        label: "macOS target (x86_64)",
    },
    Dependency {
        name: "rustc",
        args: &["--target", "x86_64-apple-darwin", "--version"],
        label: "macOS target (aarch64)",
    },
];
