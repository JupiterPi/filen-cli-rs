use std::fmt::Display;

pub struct RemotePath(String);

impl RemotePath {
    pub fn new(path: &str) -> Self {
        RemotePath(String::from("/") + path.trim_start_matches("/"))
    }

    /// Navigate to a new path.
    /// If the path starts with '/', it replaces the current path, otherwise it interprets it as a relative path.
    /// Supports ".." and ".".
    pub fn navigate(&self, navigation: &str) -> Self {
        if navigation.starts_with("/") {
            RemotePath::new(navigation)
        } else {
            let mut new_path = self.0.clone();
            navigation.split("/").for_each(|part| {
                if part == ".." {
                    let (parent, _) = new_path.rsplit_once('/').unwrap_or(("", ""));
                    new_path = parent.to_string();
                } else if part != "." && !part.is_empty() {
                    if !new_path.ends_with("/") {
                        new_path.push('/');
                    }
                    new_path.push_str(part);
                }
            });
            RemotePath(new_path)
        }
    }
}

impl Display for RemotePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remote_path_navigate() {
        let path = RemotePath::new("/root/dir");
        assert_eq!(path.navigate("subdir").0, "/root/dir/subdir");
        assert_eq!(path.navigate("..").0, "/root");
        assert_eq!(path.navigate("../..").0, "");
        assert_eq!(path.navigate("./file.txt").0, "/root/dir/file.txt");
        assert_eq!(path.navigate("./../notthedir/.././adir").0, "/root/adir"); // complex
        assert_eq!(path.navigate("../../..").0, ""); // root has no parent
    }
}