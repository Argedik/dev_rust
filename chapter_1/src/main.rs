use std::collections::HashSet;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // let root_path = "C:\\Users\\enes.gedik\\Desktop\\fe\\besin-uygulamasi";
    let root_path = "C:\\Users\\enes.gedik\\Desktop\\fe\\besin-uygulamasi\\fronten\\web";

    let mut output = Vec::new(); // Sadece dosya yollarını saklamak için bir koleksiyon
    let mut ignored = HashSet::new(); // Hariç tutulacak dosya ve klasörlerin seti

    // Önceden belirlenmiş gereksiz klasör ve dosyalar
    ignored.insert("target".to_string());
    ignored.insert("incremental".to_string());
    ignored.insert(".git".to_string());
    ignored.insert(".gitignore".to_string());

    list_files_only(root_path, root_path, &mut ignored, &mut output);

    // Tüm dosya yollarını yazdır
    for file in output {
        println!("{}", file);
    }
}

/// Belirtilen dizindeki dosyaları listeler
fn list_files_only(
    path: &str,
    root: &str,
    ignored: &mut HashSet<String>,
    output: &mut Vec<String>,
) {
    if let Ok(entries) = fs::read_dir(path) {
        // `.gitignore` dosyasını kontrol et
        let gitignore_path = format!("{}/.gitignore", path.replace("\\", "/"));
        if Path::new(&gitignore_path).exists() {
            update_ignored_from_gitignore(&gitignore_path, ignored, path);
        }

        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name().into_string().unwrap();
                let file_path = entry.path().to_str().unwrap().replace("\\", "/");

                // Önceden belirlenmiş ve `.gitignore` ile hariç tutulan dosya/klasörleri atla
                if ignored.contains(&file_name) || ignored.contains(&file_path) {
                    continue;
                }

                let file_type = entry.file_type().unwrap();
                // Eğer dosyaysa
                if file_type.is_file() {
                    let relative_path = file_path.replace(root, "besin-uygulamasi");
                    output.push(relative_path); // Dosya yolunu sakla
                }
                // Eğer klasörse
                else if file_type.is_dir() {
                    // Alt klasörleri taramaya devam et
                    list_files_only(&file_path, root, ignored, output);
                }
            }
        }
    }
}

/// `.gitignore` dosyasını okuyup, hariç tutulacak dosya/klasörleri ekler
fn update_ignored_from_gitignore(
    gitignore_path: &str,
    ignored: &mut HashSet<String>,
    base_path: &str,
) {
    if let Ok(file) = fs::File::open(gitignore_path) {
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            if let Ok(mut line) = line {
                // Yorumları ve boş satırları atla
                line = line.trim().to_string();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }

                // Satırı tam yol haline getir ve `ignored` setine ekle
                let full_path = format!("{}/{}", base_path.replace("\\", "/"), line);
                ignored.insert(full_path);
            }
        }
    }
}
