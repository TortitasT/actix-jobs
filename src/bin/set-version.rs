use std::io::Read;

fn main() {
    let new_version = std::env::args().nth(1).expect("no version given");

    let mut file = std::fs::File::open("Cargo.toml").expect("failed to open Cargo.toml");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("failed to read Cargo.toml");

    let line_where_package_is = contents
        .lines()
        .enumerate()
        .find(|(_, line)| line.contains("[package]"))
        .expect("failed to find package name")
        .0;

    let first_version_found_line = contents
        .lines()
        .enumerate()
        .skip(line_where_package_is)
        .find(|(_, line)| line.contains("version = "))
        .expect("failed to find version")
        .0;

    let mut new_contents = contents.clone();
    let new_version = format!("version = \"{}\"", new_version);

    replace_line(&mut new_contents, first_version_found_line, &new_version);

    std::fs::write("Cargo.toml", new_contents).expect("failed to write Cargo.toml");
}

fn replace_line(contents: &mut String, line: usize, new_line: &str) {
    let mut lines = contents
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<_>>();

    lines[line] = new_line.to_string();

    *contents = lines.join("\n");
    *contents += "\n";
}
