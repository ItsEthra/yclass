fn main() {
    let line = dbg!(include_str!("./Cargo.toml"))
        .lines()
        .find(|l| l.starts_with("version = "))
        .unwrap()
        .trim();

    let pos = line.rfind(' ').unwrap();
    let version = &line[pos + 2..line.len() - 1];

    println!("cargo:rustc-env=YCLASS_VERSION={}", version);
    println!("cargo:rerun-if-changed=./Cargo.toml");
}
