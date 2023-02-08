use std::{io::BufRead, process};

fn main() -> Result<(), std::io::Error> {
    let mut count = 1;

    loop {
        let mut child = process::Command::new("cmd")
            .args(["/c", "call", "D:\\gits\\doomemacs\\bin\\doom.cmd", "sync"])
            .stdout(process::Stdio::piped())
            .spawn()
            .expect("Failed to exec doom");

        let stdout = child.stdout.as_mut().ok_or_else(|| {
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

        let output = child.wait_with_output().unwrap();
        println!("{}", output.status);

        if output.status.success() {
            break;
        }

        println!("-------------------------------, times: {}", count);
        count = count + 1;
        std::thread::sleep(std::time::Duration::from_secs(3));
    }

    Ok(())
}
