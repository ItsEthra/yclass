#![allow(dead_code)]

use bytemuck::{bytes_of, bytes_of_mut, Pod};
use memflex::external::OwnedProcess;
use std::{error::Error, result};

type Result<T> = result::Result<T, Box<dyn Error>>;

pub trait ProcessInterface: Send + Sync + 'static {
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
}
