use std::path::Path;
/// Shows if an archive exists or not, returns a boolean.
pub fn exists(route: Option<&str>) -> bool{
    // If the archive exists, it returns true, if not, it returns false. 
    match route{
        Some(path) => {
            Path::new(&path).exists()
        },
        None => false 
    }
}
