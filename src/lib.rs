struct HtmlParser<'a> {
    dom: &'a tl::VDom<'a>,
    parser: &'a tl::Parser<'a>,
}

impl HtmlParser<'_> {
    fn new<'a>(dom: &'a tl::VDom<'a>, parser: &'a tl::Parser<'a>) -> HtmlParser {
        HtmlParser { dom, parser }
    }

    fn get_element_intext(&self, selector: &str) -> Option<String> {
        let node_option = self
            .dom
            .query_selector(selector)
            .and_then(|mut iter| iter.next());
        if let Some(node) = node_option {
            Some(
                node.get(self.parser)
                    .unwrap()
                    .inner_text(self.parser)
                    .to_string(),
            )
        } else {
            None
        }
    }

    fn get_element_attr(&self, selector: &str, attr: &str) -> Option<String> {
        let node_option = self
            .dom
            .query_selector(selector)
            .and_then(|mut iter| iter.next());
        if let Some(node) = node_option {
            Some(
                node.get(self.parser)
                    .unwrap()
                    .as_tag()
                    .unwrap()
                    .attributes()
                    .get(attr)
                    .flatten()
                    .unwrap()
                    .try_as_utf8_str()
                    .unwrap()
                    .to_string(),
            )
        } else {
            None
        }
    }

    fn get_title(&self) -> Option<String> {
        if let Some(value) = self.get_element_intext("title") {
            Some(value)
        } else if let Some(value) = self.get_element_attr("meta[property*=title]", "content") {
            Some(value)
        } else {
            None
        }
    }

    fn get_description(&self) -> Option<String> {
        if let Some(value) = self.get_element_attr("meta[description]", "content") {
            Some(value)
        } else if let Some(value) = self.get_element_attr("meta[property*=description]", "content")
        {
            Some(value)
        } else if let Some(value) = self.get_element_attr("meta[description]", "description") {
            Some(value)
        } else {
            None
        }
    }

    fn get_canonical(&self) -> Option<String> {
        self.get_element_attr("link[rel=canonical]", "href")
    }
}

#[derive(Debug)]
pub struct PageInfo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub canonical: Option<String>,
}

impl PageInfo {
    pub fn from_str(input: &str) -> PageInfo {
        let dom = tl::parse(&input, tl::ParserOptions::default()).unwrap();
        let parser = dom.parser();
        let html_parser = HtmlParser::new(&dom, parser);
        let title = html_parser.get_title();
        let description = html_parser.get_description();
        let canonical = html_parser.get_canonical();
        PageInfo {
            title,
            description,
            canonical,
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
    fn test_title() {
        let page_info = parse_from_test_str();
        assert_eq!(
            page_info.title,
            Some(
                "İngiliz Dizileri: ABD Yapımlarından Sıkılanlara 35 İngiliz Dizisi - CEOtudent"
                    .to_string()
            )
        );
    }

    #[test]
    fn test_description() {
        let page_info = parse_from_test_str();
        assert_eq!(
            page_info.description,
            Some(
                "ABD yapımı dizilerden sıkıldıysanız doğru yerdesiniz. İngiliz dizileri listemizle Birleşik Krallığa doğru heyecanlı bir yolculuğa çıkıyoruz."
                    .to_string()
            )
        );
    }

    #[test]
    fn test_canonical() {
        let page_info = parse_from_test_str();
        assert_eq!(
            page_info.canonical,
            Some("https://ceotudent.com/ingiliz-dizileri".to_string())
        );
    }
}
