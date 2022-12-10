use super::{bytes_to_value, SearchOptions, SearchResult};
use crate::{process::Process, thread_pool::ThreadPool};
use parking_lot::{Mutex, RwLock};
use std::sync::{
    atomic::{AtomicU16, Ordering},
    Arc,
};

pub(crate) struct ScannerState {
    results: Arc<Mutex<Vec<SearchResult>>>,
    counter: Arc<AtomicU16>,
    active: bool,
}

impl ScannerState {
    pub fn new() -> Self {
        Self {
            counter: Arc::default(),
            results: Arc::default(),
            active: false,
        }
    }

    pub fn begin(
        &mut self,
        process: &Arc<RwLock<Option<Process>>>,
        options: SearchOptions,
        pool: &Arc<ThreadPool>,
    ) {
        self.active = true;

        recursive_first_search(
            self.counter.clone(),
            pool.clone(),
            process.clone(),
            self.results.clone(),
            options,
        );
    }
}

fn recursive_first_search(
    counter: Arc<AtomicU16>,
    pool: Arc<ThreadPool>,
    process: Arc<RwLock<Option<Process>>>,
    results: Arc<Mutex<Vec<SearchResult>>>,
    opts: SearchOptions,
) {
    counter.fetch_add(1, Ordering::SeqCst);

    if opts.depth == 0 {
        return;
    }

    let start = opts.address
        + if opts.address % opts.alignment == 0 {
            0
        } else {
            opts.alignment - opts.address % opts.alignment
        };

    for address in (start..start + opts.struct_size).step_by(opts.alignment) {
        let mut buf = [0; 8];
        process.read().as_ref().unwrap().read(address, &mut buf[..]);

        if address % 8 == 0
            && process
                .read()
                .as_ref()
                .unwrap()
                .can_read(usize::from_ne_bytes(buf))
        {
            pool.spawn({
                let pool = pool.clone();
                let results = results.clone();
                let offsets = opts.offsets.clone();
                let process = process.clone();
                let counter = counter.clone();

                move || {
                    recursive_first_search(
                        counter,
                        pool,
                        process,
                        results,
                        SearchOptions {
                            offsets: Arc::new(
                                offsets.iter().copied().chain([address - start]).collect(),
                            ),
                            address: usize::from_ne_bytes(buf),
                            struct_size: opts.struct_size,
                            alignment: opts.alignment,
                            depth: opts.depth - 1,
                            value: opts.value,
                        },
                    );
                }
            })
        }

        let value = bytes_to_value(&buf, opts.value.kind());

        if value == opts.value {
            results.lock().push(SearchResult {
                parent_offsets: opts.offsets.clone(),
                offset: address - start,
                last_value: value,
            });
        }
    }

    counter.fetch_sub(1, Ordering::SeqCst);
}
