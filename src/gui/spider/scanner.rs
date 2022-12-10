use super::{bytes_to_value, SearchOptions, SearchResult};
use crate::process::Process;
use parking_lot::{Mutex, RwLock};
use std::{
    sync::{
        atomic::{AtomicU16, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

pub(crate) struct ScannerState {
    results: Arc<Mutex<Vec<SearchResult>>>,
    counter: Arc<AtomicU16>,
    start: Instant,
    active: bool,
}

pub(crate) enum ScannerReport {
    Finshed(Duration, Vec<SearchResult>),
    InProgress,
    Idle,
}

impl ScannerState {
    pub fn new() -> Self {
        Self {
            counter: Arc::default(),
            results: Arc::default(),
            start: Instant::now(),
            active: false,
        }
    }

    pub fn begin(&mut self, process: &Arc<RwLock<Option<Process>>>, options: SearchOptions) {
        self.active = true;
        self.start = Instant::now();
        self.counter.store(0, Ordering::SeqCst);

        recursive_first_search(
            self.counter.clone(),
            process.clone(),
            self.results.clone(),
            options,
        );
    }

    pub fn try_take(&mut self) -> ScannerReport {
        if self.active {
            if self.counter.load(Ordering::SeqCst) == 0 {
                self.active = false;
                ScannerReport::Finshed(
                    self.start.elapsed(),
                    std::mem::take(&mut *self.results.lock()),
                )
            } else {
                ScannerReport::InProgress
            }
        } else {
            ScannerReport::Idle
        }
    }
}

fn recursive_first_search(
    counter: Arc<AtomicU16>,
    process: Arc<RwLock<Option<Process>>>,
    results: Arc<Mutex<Vec<SearchResult>>>,
    opts: SearchOptions,
) {
    if opts.depth == 0 {
        return;
    }

    counter.fetch_add(1, Ordering::SeqCst);

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
            rayon::spawn({
                let results = results.clone();
                let offsets = opts.offsets.clone();
                let process = process.clone();
                let counter = counter.clone();

                move || {
                    recursive_first_search(
                        counter,
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
