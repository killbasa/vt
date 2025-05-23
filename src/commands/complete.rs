use std::io;

use anyhow::Result;
use clap::{Args, CommandFactory};
use clap_complete::{Shell, generate};

use crate::Cli as RootCli;

/// Generate autocompletions
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    /// The shell to generate autocompletions for
    #[arg(value_enum)]
    shell: Shell,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        let mut cmd = RootCli::command();
        let bin_name = cmd.get_name().to_string();

        generate(self.shell, &mut cmd, bin_name, &mut io::stdout());

        Ok(())
    }
}
