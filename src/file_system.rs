use std::collections::{hash_map::Entry, HashMap};

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub size: u64,
}

impl File {
    pub fn new(name: String, size: u64) -> Self {
        Self { name, size }
    }
}

#[derive(Debug, Clone)]
pub struct Folder {
    pub name: String,
    pub files: HashMap<String, File>,
    pub sub_folders: HashMap<String, Folder>,
    pub size: u64,
}

pub fn collect_data(path: impl Into<std::path::PathBuf>) -> anyhow::Result<Folder> {
    let path: std::path::PathBuf = path.into();

    let path_file_name = match path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        // TODO: Yeah, don't know, but it can fail
        None => todo!(),
    };

    let mut folder_info = Folder {
        name: path_file_name,
        files: HashMap::new(),
        sub_folders: HashMap::new(),
        size: 0
    };

    let files = std::fs::read_dir(path)?;

    for file in files {
        let file = file?;
        let metadata = file.metadata()?;
        let file_name = file.file_name().to_string_lossy().to_string();

        /*
            Use Entry API for HashMap:
            This avoids duplicate key lookups in the HashMaps.
            I can also improve performance when inserting or updating entries in the files and sub_folders HashMaps.
        */
        if metadata.is_dir() {
            let sub_folders = collect_data(file.path())?;
            folder_info.size += sub_folders.size;
            match folder_info.sub_folders.entry(file_name) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().size += sub_folders.size;
                }
                Entry::Vacant(entry) => {
                    entry.insert(sub_folders);
                }
            }
        } else {
            folder_info.size += metadata.len();
            match folder_info.files.entry(file_name.clone()) {
                Entry::Occupied(_) => {
                    continue;
                },
                Entry::Vacant(entry) => {
                    entry.insert(File::new(file_name, metadata.len()));
                }
            }
        }
    }

    Ok(folder_info)
}