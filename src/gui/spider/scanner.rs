use super::{bytes_to_value, SearchOptions, SearchResult};
use crate::{process::Process, thread_pool::ThreadPool};
use parking_lot::Mutex;
use std::sync::Arc;

pub struct ScannerState {}

impl ScannerState {
    pub fn new() {}
}

fn recursive_first_search(
    pool: Arc<ThreadPool>,
    process: Arc<Process>,
    results: Arc<Mutex<Vec<SearchResult>>>,
    opts: SearchOptions,
) {
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
        process.read(address, &mut buf[..]);

        if address % 8 == 0 && process.can_read(usize::from_ne_bytes(buf)) {
            pool.spawn({
                let pool = pool.clone();
                let results = results.clone();
                let offsets = opts.offsets.clone();
                let process = process.clone();

                move || {
                    recursive_first_search(
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
}
