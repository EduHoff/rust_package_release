pub struct BuildTarget {
    pub name: &'static str,
    pub label: &'static str,
}

pub const TARGETS: &[BuildTarget] = &[
    BuildTarget {
        name: "x86_64-unknown-linux-musl",
        label: "linux-x86_64",
    },
    BuildTarget {
        name: "aarch64-unknown-linux-musl",
        label: "linux-aarch64",
    },
    BuildTarget {
        name: "x86_64-pc-windows-gnu",
        label: "windows-x86_64",
    },
    BuildTarget {
        name: "x86_64-unknown-freebsd",
        label: "freebsd-x86_64",
    },
];

pub const MAC_TARGETS: &[BuildTarget] = &[
    BuildTarget {
        name: "x86_64-apple-darwin",
        label: "apple-x86_64",
    },
    BuildTarget {
        name: "aarch64-apple-darwin",
        label: "apple-aarch64",
    },
];
