use std::path::PathBuf;

#[derive(Debug)]
pub struct FileChecker {
    includes: Vec<PathBuf>,
}

impl FileChecker {
    pub fn new() -> Result<Self, std::io::Error> {
        let mut includes = vec![];
        let compiler = cc::Build::new().get_compiler();
        let result = compiler
            .to_command()
            .args(["-E", "-v", "-xc", "/dev/null"])
            .output();
        if let Ok(output) = result {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let start_index = stderr.find("#include <...> search starts here:\n").unwrap()
                + "#include <...> search starts here:\n".len();
            let end_index = stderr.find("End of search list.").unwrap();
            let include_dirs_str = &stderr[start_index..end_index];
            let include_dirs = include_dirs_str.split('\n');
            for dir in include_dirs {
                if !dir.is_empty() {
                    let include_dir = PathBuf::from(dir.trim());
                    if include_dir.exists() {
                        includes.push(include_dir);
                    }
                }
            }
        }
        Ok(Self { includes })
    }

    pub fn check_header(&self, header: &str) -> bool {
        for include_dir in &self.includes {
            let mut header_path = std::path::PathBuf::new();
            header_path.push(include_dir);
            header_path.push(header);

            if file_exists(&header_path) {
                return true;
            }
        }
        false
    }
}

fn file_exists(path: &std::path::Path) -> bool {
    path.exists() && path.is_file()
}
