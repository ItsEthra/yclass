use memflex::external::ProcessIterator;

#[derive(serde::Serialize)]
pub struct Process {
    name: String,
    id: u32,
}

#[tauri::command]
pub fn fetch_all_processes(filter: String) -> Vec<Process> {
    ProcessIterator::new()
        .map(|pi| {
            pi.filter_map(|pe| {
                if filter.is_empty() || pe.name.to_lowercase().contains(&filter) {
                    Some(Process {
                        name: pe.name,
                        id: pe.id,
                    })
                } else {
                    None
                }
            })
            .collect()
        })
        .unwrap_or_default()
}
