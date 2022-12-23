#[cfg(test)]
mod tests {
    use metascraper::MetaScraper;

    fn parse_test_str() -> MetaScraper {
        let html = include_str!("test.html");
        MetaScraper::parse(html)
    }

    #[test]
    fn test_page() {
        let page_info = parse_test_str();
        assert_eq!(page_info.title, Some("Title".to_string()));
        assert_eq!(page_info.language, Some("en".to_string()));
        assert_eq!(page_info.description, Some("Description".to_string()));
        assert_eq!(
            page_info.canonical,
            Some("https://mehmetcan.sahin.dev".to_string())
        );
        assert_eq!(page_info.rss, Some("rss.xml".to_string()));
    }

    #[test]
    fn test_from_website() {
        let resp = reqwest::blocking::get("https://mehmetcan.sahin.dev/")
            .unwrap()
            .text()
            .unwrap();
        let page_info = MetaScraper::parse(&resp);
        assert_eq!(page_info.title, Some("Mehmetcan Şahin".to_string()));
        assert_eq!(page_info.language, Some("en".to_string()));
        assert_eq!(page_info.description, Some("Personal Page".to_string()));
    }
}
