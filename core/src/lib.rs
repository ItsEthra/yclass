use error::Result;
use serde::Serialize;

mod error;

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
