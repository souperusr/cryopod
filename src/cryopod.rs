use crate::error::{Error, Result};

use std::io::prelude::*;
use std::process::Command;
use std::path::PathBuf;

pub struct Project {
    path: PathBuf,
}

pub struct Cryopod ();

impl Cryopod {
    pub fn new() -> Result<Self> {
        // Enforce invariants
        Ok(Self ())
    }
    pub fn develop(&self) -> Result<()> {
        return Ok(());
    }
}

// pub fn enter() -> Result<(), std::io::Error> {
//     println!("hi");
//     return Ok(());
// }
