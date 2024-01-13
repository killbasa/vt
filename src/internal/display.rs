use anyhow::Result;
use std::process::{Child, Command, Stdio};

use super::programs;

fn _echo(content: String) -> Child {
    Command::new("echo")
        .arg(content)
        .stdout(Stdio::piped())
        .spawn()
        .expect("echo failed")
}

#[allow(dead_code)]
pub fn with_less(content: String) -> Result<()> {
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
pub fn with_more(content: String) -> Result<()> {
    let echo = _echo(content);

    Command::new("more")
        .stdin(Stdio::from(echo.stdout.unwrap()))
        .spawn()
        .expect("command failed")
        .wait()
        .expect("wait failed");

    Ok(())
}

pub fn with_print(content: String) {
    println!("{}", content);
}

#[allow(dead_code)]
pub fn with_any(content: String) -> Result<()> {
    if programs::is_program_in_path("less") {
        with_less(content)?;
    } else if programs::is_program_in_path("more") {
        with_more(content)?;
    } else {
        with_print(content);
    }

    Ok(())
}
