#[cfg(test)]
mod tests {
    use crate::files;
    use crate::files::files_grouping;

    use std::collections::HashMap;
    use std::path::PathBuf;

    #[test]
    fn test_image_type() {
        assert_eq!(files::file_type("png"), "Images");
    }

    #[test]
    fn test_video_type() {
        assert_eq!(files::file_type("mp4"), "Videos");
    }

    #[test]
    fn test_unknown_type() {
        assert_eq!(files::file_type("abc"), "Unknown");
    }

    #[test]
    fn test_file_grouping_images() {
        let mut groups = HashMap::new();

        let path = PathBuf::from("cat.png");

        files_grouping(path, &mut groups);

        assert_eq!(groups.contains_key("Images"), true);

        let images = groups.get("Images").unwrap();

        assert_eq!(images.len(), 1)
    }

    #[test]
    fn test_file_grouping_many() {
        let mut groups = HashMap::new();

        let path1 = PathBuf::from("cat.png");
        let path2 = PathBuf::from("animal.png");
        let path3 = PathBuf::from("animal.mp4");

        files_grouping(path1, &mut groups);
        files_grouping(path2, &mut groups);
        files_grouping(path3, &mut groups);

        assert_eq!(groups.contains_key("Images"), true);
        assert_eq!(groups.contains_key("Videos"), true);

        let images = groups.get("Images").unwrap();

        assert_eq!(images.len(), 2)
    }
}
