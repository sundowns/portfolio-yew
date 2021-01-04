mod content {
    pub enum ContentType {
        Video,
        Screenshot,
    }
    pub struct ProjectContent {
        content_type: ContentType,
        alt_text: String,
        url: String,
    }

    impl ProjectContent {
        // Another static method, taking two arguments:
        fn new(content_type: ContentType, alt_text: &str, url: &str) -> ProjectContent {
            ProjectContent {
                content_type: content_type,
                alt_text: alt_text.to_owned(),
                url: url.to_owned(),
            }
        }
    }
}
