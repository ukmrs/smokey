//! handles  copying the storage
//! from ./storage
//! linux -> /home/user/.local/share/smokey

use directories_next::ProjectDirs;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn main() {
    let source_storage = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("storage");

    let local_storage = ProjectDirs::from("pl", "ukmrs", "smokey")
        .expect("no valid home directory could be found")
        .data_dir()
        .to_path_buf();

    copy_dir_recursively(&source_storage, &local_storage).expect("couldnt install word packs");
}

// thanks doug
// https://stackoverflow.com/users/353820/doug
pub fn copy_dir_recursively<U: AsRef<Path>, V: AsRef<Path>>(
    from: U,
    to: V,
) -> Result<(), std::io::Error> {
    let mut stack = vec![PathBuf::from(from.as_ref())];

    let output_root = PathBuf::from(to.as_ref());
    let input_root = PathBuf::from(from.as_ref()).components().count();

    while let Some(working_path) = stack.pop() {
        let src: PathBuf = working_path.components().skip(input_root).collect();
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if fs::metadata(&dest).is_err() {
            fs::create_dir_all(&dest)?;
        }

        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if let Some(filename) = path.file_name() {
                let dest_path = dest.join(filename);
                fs::copy(&path, &dest_path)?;
            }
        }
    }

    Ok(())
}
