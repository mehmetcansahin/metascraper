struct HtmlParser<'a> {
    dom: &'a tl::VDom<'a>,
    parser: &'a tl::Parser<'a>,
}

impl HtmlParser<'_> {
    fn new<'a>(dom: &'a tl::VDom<'a>, parser: &'a tl::Parser<'a>) -> HtmlParser<'a> {
        HtmlParser { dom, parser }
    }

    fn metatags(&self) -> Option<Vec<Metatag>> {
        let mut metatags: Vec<Metatag> = Vec::new();
        let query_sellector_iter = self.dom.query_selector(r#"meta"#)?;
        for node_handle in query_sellector_iter {
            let node = node_handle.get(self.parser)?;
            if let Some(tag) = node.as_tag() {
                let name = tag
                    .attributes()
                    .get("name")
                    .or_else(|| tag.attributes().get("property"))
                    .flatten()
                    .and_then(|x| Some(x.as_utf8_str().to_string()));

                let content = tag
                    .attributes()
                    .get("content")
                    .or_else(|| tag.attributes().get("description"))
                    .flatten()
                    .and_then(|x| Some(x.as_utf8_str().to_string()));

                if name.is_some() && content.is_some() {
                    let nt = Metatag {
                        name: name?,
                        content: content?,
                    };
                    metatags.push(nt);
                }
            }
        }
        Some(metatags)
    }

    fn get_intext(&self, selector: &str) -> Option<String> {
        self.dom
            .query_selector(selector)
            .and_then(|mut iter| iter.next())
            .and_then(|node_handle| node_handle.get(self.parser))
            .and_then(|node| Some(node.inner_text(self.parser).to_string()))
    }

    fn get_attribute(&self, selector: &str, attr: &str) -> Option<String> {
        self.dom
            .query_selector(selector)
            .and_then(|mut iter| iter.next())
            .and_then(|node_handle| node_handle.get(self.parser))
            .and_then(|node| node.as_tag())
            .and_then(|html_tag| html_tag.attributes().get(attr).flatten())
            .and_then(|bytes| Some(bytes.as_utf8_str().to_string()))
    }

    fn rss(&self) -> Option<String> {
        self.get_attribute(r#"link[type*="rss"]"#, "href")
    }

    fn title(&self) -> Option<String> {
        self.get_intext("title")
            .or_else(|| self.get_attribute("meta[property*=title]", "content"))
    }

    fn description(&self) -> Option<String> {
        self.get_attribute(r#"meta[name="description"]"#, "content")
            .or_else(|| self.get_attribute("meta[property*=description]", "content"))
            .or_else(|| self.get_attribute("meta[description]", "description"))
    }

    fn canonical(&self) -> Option<String> {
        self.get_attribute("link[rel=canonical]", "href")
    }

    fn language(&self) -> Option<String> {
        self.get_attribute("html", "lang")
    }
}

#[derive(Debug)]
pub struct Metatag {
    pub name: String,
    pub content: String,
}

#[derive(Debug)]
pub struct PageInfo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub canonical: Option<String>,
    pub language: Option<String>,
    pub rss: Option<String>,
    pub metatags: Option<Vec<Metatag>>,
}

impl PageInfo {
    pub fn from_str(input: &str) -> PageInfo {
        let dom = tl::parse(&input, tl::ParserOptions::default()).unwrap();
        let parser = dom.parser();
        let html_parser = HtmlParser::new(&dom, parser);
        PageInfo {
            title: html_parser.title(),
            description: html_parser.description(),
            canonical: html_parser.canonical(),
            language: html_parser.language(),
            rss: html_parser.rss(),
            metatags: html_parser.metatags(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_from_test_str() -> PageInfo {
        let html = include_str!("test.html");
        PageInfo::from_str(html)
    }

    #[test]
    fn test_page() {
        let page_info = parse_from_test_str();
        assert_eq!(page_info.title, Some("Pageinfo".to_string()));
        assert_eq!(page_info.language, Some("en".to_string()));
        assert_eq!(
            page_info.description,
            Some("Pageinfo description".to_string())
        );
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
        let page_info = PageInfo::from_str(&resp);
        assert_eq!(page_info.title, Some("Mehmetcan Şahin".to_string()));
        assert_eq!(page_info.language, Some("en".to_string()));
        assert_eq!(page_info.description, Some("Personal Page".to_string()));
    }
}
