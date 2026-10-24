use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub fn write(inputs: &Path, out: &Path) -> io::Result<Vec<PathBuf>> {
    let catalog = super::models::Catalog::load(inputs)?;
    fn collect(directory: &Path, directories: &mut Vec<PathBuf>) -> io::Result<()> {
        directories.push(directory.to_owned());
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();
            if entry.file_type()?.is_dir() {
                collect(&path, directories)?;
            }
        }
        Ok(())
    }
    let root = inputs.join("peripherals");
    let mut names = catalog
        .entries
        .values()
        .filter(|entry| entry.source.is_some())
        .map(|entry| entry.driver.clone())
        .collect::<Vec<_>>();
    let mut directories = Vec::new();
    collect(&root, &mut directories)?;
    names.sort();
    names.dedup();
    directories.sort();
    directories.push(root.join("models.json"));
    let values = names
        .iter()
        .map(|name| format!("{name:?}"))
        .collect::<Vec<_>>()
        .join(",");
    fs::create_dir_all(out)?;
    fs::write(
        out.join("meta_peripherals.rs"),
        format!(
            "/// List of all nxp-pac peripherals, whether they are used or not.\npub const META_PERIPHERALS: &[&str] = &[{values}];\n"
        ),
    )?;
    Ok(directories)
}
