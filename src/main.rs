use std::io::{self, Write};
use std::path::Path;
use file_explorer_windows::core::file_metadata::{ClickResult, FileMetadata};

fn main() {
    let mut start_path = String::from("C:\\Users");
    let mut list = FileMetadata::list_all_by_path(Path::new(&start_path));

    loop {
        println!("PASTA ATUAL: {}", start_path);

        for (i, item) in list.iter().enumerate() {
            let prefixo = if item.is_dir() { "[DIR]" } else { "[FILE]" };
            println!("{}: {} {}", i, prefixo, item.name);
        }

        print!("\nDigite o número para abrir (ou 'q' para sair): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "q" {break;}

        if let Ok(idx) = input.parse::<usize>() {
            if let Some(escolhido) = list.get(idx) {
                match escolhido.open() {
                    ClickResult::OpenedFolder(new_items) => {
                        start_path = escolhido.path.to_string_lossy().to_string();
                        println!("Entrando em...");
                        list = new_items;
                    },
                    ClickResult::OpenedFile => println!("Arquivo aberto no Windows. Continuando na mesma pasta..."),
                    ClickResult::Error(e) => eprintln!("Erro ao abrir: {}", e),
                }
            }
        } else {
            println!("Invalid Entry");
        }
    }
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