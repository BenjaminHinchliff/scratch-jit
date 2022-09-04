use std::{fs::File, path::Path};

use temp_dir::TempDir;

pub fn unpack_project<P>(path: P) -> Result<TempDir, std::io::Error>
where
    P: AsRef<Path>,
{
    let unpack_dir = TempDir::new()?;

    let file = File::open(path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    archive.extract(unpack_dir.path())?;

    Ok(unpack_dir)
}
