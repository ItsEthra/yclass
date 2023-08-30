use memflex::external::find_process_by_id;
use serde::Serialize;

mod eval;
pub use eval::*;
mod interface;
pub use interface::*;
mod error;
pub use error::*;

#[derive(Serialize)]
pub struct ProcessEntry {
    pub name: String,
    pub id: u32,
    // TODO: Icon
}

pub fn fetch_processes() -> Result<Vec<ProcessEntry>> {
    let iter = memflex::external::ProcessIterator::new()?;
    Ok(iter
        .map(|e| ProcessEntry {
            name: e.name,
            id: e.id,
        })
        .collect())
}

pub fn attach(pid: u32) -> Result<Box<dyn ProcessInterface>> {
    let proc = find_process_by_id(pid)?;
    Ok(Box::new(proc) as Box<dyn ProcessInterface>)
}
