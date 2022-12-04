use libloading::Library;
use memflex::external::OwnedProcess;
use std::{error::Error, fs};

pub struct ManagedExtension {
    #[allow(dead_code)]
    lib: Library,
    // process id
    pid: u32,

    attach: fn(u32) -> u32,
    read: fn(usize, *mut u8, usize) -> u32,
    detach: fn(),
}

impl Drop for ManagedExtension {
    fn drop(&mut self) {
        (self.detach)();
    }
}

pub enum Process {
    Internal(OwnedProcess),
    Managed(ManagedExtension),
}

impl Process {
    pub fn attach(pid: u32) -> Result<Self, Box<dyn Error>> {
        Ok(if fs::metadata("plugin.ycpl").is_ok() {
            let lib = unsafe { Library::new("plugin.yclp")? };
            let attach = unsafe { *lib.get::<fn(u32) -> u32>(b"yc_attach")? };
            let read = unsafe { *lib.get::<fn(usize, *mut u8, usize) -> u32>(b"yc_read")? };
            let detach = unsafe { *lib.get::<fn()>(b"yc_detach")? };

            let ext = ManagedExtension {
                pid,
                lib,
                attach,
                read,
                detach,
            };

            (ext.attach)(pid);

            Self::Managed(ext)
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

            Self::Internal(proc)
        })
    }

    pub fn read(&self, address: usize, buf: &mut [u8]) -> bool {
        match self {
            // TODO(ItsEthra): Proper error handling maybe?.
            Self::Internal(op) => op.read_buf(address, buf).is_ok(),
            Self::Managed(ext) => (ext.read)(address, buf.as_mut_ptr(), buf.len()) == 0,
        }
    }

    pub fn id(&self) -> u32 {
        match self {
            Self::Internal(op) => op.id(),
            Self::Managed(ext) => ext.pid,
        }
    }

    pub fn name(&self) -> String {
        match self {
            Self::Internal(op) => op.name(),
            Self::Managed(_) => unimplemented!(),
        }
    }
}
