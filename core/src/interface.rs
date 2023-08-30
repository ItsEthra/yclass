#![allow(dead_code)]

use bytemuck::{bytes_of, bytes_of_mut, Pod};
use memflex::{external::OwnedProcess, types::ModuleInfoWithName};
use std::{error, result};

use crate::Error;

type Result<T> = result::Result<T, Box<dyn error::Error>>;

pub trait ProcessInterface: Send + Sync + 'static {
    fn module_base(&self, name: Option<&str>) -> Result<usize>;
    fn read_buf(&self, address: usize, buf: &mut [u8]) -> Result<()>;
    fn write_buf(&self, address: usize, buf: &[u8]) -> Result<()>;
}

impl dyn ProcessInterface {
    fn read<T: Pod>(&self, address: usize) -> Result<T> {
        let mut temp: T = T::zeroed();
        self.read_buf(address, bytes_of_mut(&mut temp))
            .map(|_| temp)
    }

    fn write<T: Pod>(&self, address: usize, value: &T) -> Result<()> {
        self.write_buf(address, bytes_of(value))
    }
}

impl ProcessInterface for OwnedProcess {
    fn read_buf(&self, address: usize, buf: &mut [u8]) -> Result<()> {
        OwnedProcess::read_buf(self, address, buf)?;

        Ok(())
    }

    fn write_buf(&self, address: usize, buf: &[u8]) -> Result<()> {
        OwnedProcess::write_buf(self, address, buf)?;

        Ok(())
    }

    fn module_base(&self, name: Option<&str>) -> Result<usize> {
        // TODO: Find a way to reliably get process base on unix
        #[cfg(unix)]
        let else_base = |m: &ModuleInfoWithName| m.name == "subject";
        #[cfg(windows)]
        let else_base = |m: &ModuleInfoWithName| m.name.ends_with("exe");

        let module = self
            .modules()?
            .find(|m| name.map(|n| m.name == n).unwrap_or_else(|| else_base(m)))
            .ok_or_else(|| Error::MissingModule(name.unwrap().to_owned()))?;
        Ok(module.base as usize)
    }
}
