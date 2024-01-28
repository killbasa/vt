use anyhow::Result;
use std::process::{Child, Command, Stdio};

fn _echo(content: &str) -> Child {
    Command::new("echo")
        .arg(content)
        .stdout(Stdio::piped())
        .spawn()
        .expect("echo failed")
}

#[allow(dead_code)]
pub fn with_less(content: &str) -> Result<()> {
    let echo = _echo(content);

    Command::new("less")
        .arg("-r")
        .stdin(Stdio::from(echo.stdout.unwrap()))
        .spawn()
        .expect("command failed")
        .wait()
        .expect("wait failed");

    Ok(())
}

#[allow(dead_code)]
pub fn with_more(content: &str) -> Result<()> {
    let echo = _echo(content);

    Command::new("more")
        .stdin(Stdio::from(echo.stdout.unwrap()))
        .spawn()
        .expect("command failed")
        .wait()
        .expect("wait failed");

    Ok(())
}

pub fn with_print(content: &str) {
    println!("{}", content);
}
