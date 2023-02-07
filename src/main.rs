use std::{io::BufRead, process};

fn main() -> Result<(), std::io::Error> {
    let child = process::Command::new("cmd")
        .args([
            "/c",
            "call",
            "D:\\gits\\doomemacs\\bin\\doom.cmd",
            "install",
        ])
        .stdout(process::Stdio::piped())
        .spawn()
        .expect("Failed to exec doom");

    let stdout = child.stdout.ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            "Could not capture standard output.",
        )
    })?;

    let reader = std::io::BufReader::new(stdout);

    reader
        .lines()
        .filter_map(|line| line.ok())
        // .filter(|line| line.find("x").is_some())
        .for_each(|line| println!("{}", line));

    Ok(())
}
