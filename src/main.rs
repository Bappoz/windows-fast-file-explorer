use file_explorer_windows::core::file_metadata::FileMetadata;
use walkdir::WalkDir;

fn main() {
    let path = "src";
    
    // Get the DirEntry for the path
    if let Some(Ok(entry)) = WalkDir::new(path).into_iter().next() {
        FileMetadata::from_entry(&entry);
    }
    
    FileMetadata::list_all_by_path(path);
}











/*
let search_root = "../../../../";
    let target_folder = "softwares";
    
    let mut found_path = None;
    for entry in WalkDir::new(search_root)
        .min_depth(1)
        .max_depth(3) 
        .into_iter()
        .filter_entry(|e| !is_hidden(e)){
            
            if let Ok(ref entry) = entry {
                if entry.file_type().is_dir() {
                    if entry.file_name().to_str() == Some(target_folder) {
                        found_path = Some(entry.path().to_path_buf());
                        break;
                    }
                }
            }    
        }

    if let Some(path) = found_path {
        let mut walker = WalkDir::new(path)
            .into_iter()
            .filter_entry(|e| !is_hidden(e));
            
            while let Some(entry) = walker.next() {
                match entry {
                    Ok(entry) => {
                        if entry.file_type().is_dir() && entry.file_name() == "node_modules" {
                            walker.skip_current_dir();
                            continue;    
                        }  
                        println!("{}", entry.path().display())
                    },
                    Err(e) => eprintln!("Error reading entry: {}", e),
                }
            }
    } else {
        println!("Diretorio {} nao encontrado", target_folder);
    }
     */