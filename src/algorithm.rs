use std::fmt::Display;

use clap::ValueEnum;

#[derive(Clone, ValueEnum)]
pub enum Algorithm {
    /// Most successors first schedules
    Msf,

    /// Coffman–Graham algorithm
    Gc,
}

impl Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Algorithm::Msf => write!(f, "MSF"),
            Algorithm::Gc => write!(f, "GC"),
        }
    }
}
