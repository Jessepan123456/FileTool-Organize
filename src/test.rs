#[cfg(test)]
mod tests {
    use crate::functions::files::{custom_files_grouping, default_file_type, default_files_grouping};
    use std::collections::HashMap;
    use std::path::PathBuf;

    #[test]
    fn test_image_type() {
        assert_eq!(default_file_type("png"), "Images");
    }

    #[test]
    fn test_video_type() {
        assert_eq!(default_file_type("mp4"), "Videos");
    }

    #[test]
    fn test_unknown_type() {
        assert_eq!(default_file_type("abc"), "Unknown");
    }

    #[test]
    fn test_file_grouping_images() {
        let mut groups = HashMap::new();

        let path = PathBuf::from("cat.png");

        default_files_grouping(path, &mut groups);

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

        default_files_grouping(path1, &mut groups);
        default_files_grouping(path2, &mut groups);
        default_files_grouping(path3, &mut groups);

        assert_eq!(groups.contains_key("Images"), true);
        assert_eq!(groups.contains_key("Videos"), true);

        let images = groups.get("Images").unwrap();

        assert_eq!(images.len(), 2)
    }

    #[test]
    fn custom_grouping_test() {
        let mut groups: HashMap<String, Vec<PathBuf>> = HashMap::new();

        let mut custom: HashMap<String, Vec<String>> = HashMap::new();
        custom.insert(
            "Images".to_string(),
            vec!["png".to_string(), "jpg".to_string()],
        );

        let path = PathBuf::from("cat.png");

        custom_files_grouping(path.clone(), &mut groups, &custom);

        assert!(groups.contains_key("Images"));
        assert_eq!(groups["Images"][0], path);
    }
}
