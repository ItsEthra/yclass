use crate::config::YClassConfig;
use libloading::Library;
use memflex::external::{MemoryRegion, OwnedProcess};
use std::{error::Error, fs};

pub struct ManagedExtension {
    #[allow(dead_code)]
    lib: Library,
    // process id
    pid: u32,

    attach: fn(u32) -> u32,
    read: fn(usize, *mut u8, usize) -> u32,
    write: fn(usize, *const u8, usize) -> u32,
    can_read: fn(usize) -> bool,
    detach: fn(),
}

impl Drop for ManagedExtension {
    fn drop(&mut self) {
        (self.detach)();
    }
}

pub enum Process {
    Internal((OwnedProcess, Vec<MemoryRegion>)),
    Managed(ManagedExtension),
}

impl Process {
    pub fn attach(pid: u32, config: &YClassConfig) -> Result<Self, Box<dyn Error>> {
        let (path, modified) = (
            config
                .plugin_path
                .clone()
                .unwrap_or_else(|| "plugin.ycpl".into()),
            config.plugin_path.is_some(),
        );

        let metadata = fs::metadata(&path);
        Ok(if metadata.is_ok() {
            let lib = unsafe { Library::new(&path)? };
            let attach = unsafe { *lib.get::<fn(u32) -> u32>(b"yc_attach")? };
            let read = unsafe { *lib.get::<fn(usize, *mut u8, usize) -> u32>(b"yc_read")? };
            let write = unsafe { *lib.get::<fn(usize, *const u8, usize) -> u32>(b"yc_write")? };
            let can_read = unsafe { *lib.get::<fn(usize) -> bool>(b"yc_can_read")? };
            let detach = unsafe { *lib.get::<fn()>(b"yc_detach")? };

            let ext = ManagedExtension {
                pid,
                lib,
                attach,
                read,
                write,
                can_read,
                detach,
            };

            (ext.attach)(pid);

            Self::Managed(ext)
        } else if modified {
            #[allow(clippy::unnecessary_unwrap)]
            return Err(metadata.unwrap_err().into());
        } else {
            #[cfg(unix)]
            let proc = memflex::external::find_process_by_id(pid)?;
            #[cfg(windows)]
            let proc = {
                use memflex::types::win::{
                    PROCESS_QUERY_INFORMATION, PROCESS_VM_READ, PROCESS_VM_WRITE,
                };

                memflex::external::open_process_by_id(
                    pid,
                    false,
                    PROCESS_VM_READ | PROCESS_VM_WRITE | PROCESS_QUERY_INFORMATION,
                )?
            };

            let maps = proc.maps();
            Self::Internal((proc, maps))
        })
    }

    pub fn read(&self, address: usize, buf: &mut [u8]) {
        match self {
            // TODO(ItsEthra): Proper error handling maybe?.
            Self::Internal((op, _)) => _ = op.read_buf(address, buf),
            Self::Managed(ext) => _ = (ext.read)(address, buf.as_mut_ptr(), buf.len()),
        };
    }

    pub fn write(&self, address: usize, buf: &[u8]) {
        match self {
            // TODO(ItsEthra): Proper error handling maybe?.
            Self::Internal((op, _)) => _ = op.write_buf(address, buf),
            Self::Managed(ext) => _ = (ext.write)(address, buf.as_ptr(), buf.len()),
        };
    }

    pub fn id(&self) -> u32 {
        match self {
            Self::Internal((op, _)) => op.id(),
            Self::Managed(ext) => ext.pid,
        }
    }

    pub fn can_read(&self, address: usize) -> bool {
        match self {
            Self::Internal((_, maps)) => maps
                .iter()
                .any(|map| map.from <= address && map.to >= address && map.prot.read()),
            Self::Managed(ext) => (ext.can_read)(address),
        }
    }

    pub fn name(&self) -> String {
        match self {
            Self::Internal((op, _)) => op.name(),
            Self::Managed(_) => "[MANAGED]".into(),
        }
    }
}
