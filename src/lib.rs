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

    fn get_rss(&self) -> Option<Vec<Rss>> {
        let node_option = self.dom.query_selector(r#"link[type*="rss"]"#);
        if let Some(nodes) = node_option {
            let mut list: Vec<Rss> = Vec::new();
            for node in nodes {
                let href = node
                    .get(self.parser)
                    .unwrap()
                    .as_tag()
                    .unwrap()
                    .attributes()
                    .get("href")
                    .flatten()
                    .unwrap()
                    .try_as_utf8_str()
                    .unwrap()
                    .to_string();
                let title = node
                    .get(self.parser)
                    .unwrap()
                    .as_tag()
                    .unwrap()
                    .attributes()
                    .get("title")
                    .flatten()
                    .unwrap()
                    .try_as_utf8_str()
                    .unwrap()
                    .to_string();
                let rss = Rss {
                    href,
                    title: Some(title),
                };
                list.push(rss);
            }
            if list.len() > 0 {
                Some(list)
            } else {
                None
            }
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

    fn get_language(&self) -> Option<String> {
        self.get_element_attr("html", "lang")
    }

    fn get_metatags(&self) -> Option<Vec<Metatag>> {
        let mut metatags: Vec<Metatag> = Vec::new();
        let node_option = self.dom.query_selector(r#"meta"#);
        if let Some(nodes) = node_option {
            for node in nodes {
                let node = node.get(self.parser).unwrap();
                if let Some(tag) = node.as_tag() {
                    let name = if let Some(attr) = tag.attributes().get("name").flatten() {
                        Some(attr.as_utf8_str().to_string())
                    } else if let Some(attr) = tag.attributes().get("property").flatten() {
                        Some(attr.as_utf8_str().to_string())
                    } else {
                        None
                    };
                    let content = if let Some(attr) = tag.attributes().get("content").flatten() {
                        Some(attr.as_utf8_str().to_string())
                    } else if let Some(attr) = tag.attributes().get("description").flatten() {
                        Some(attr.as_utf8_str().to_string())
                    } else {
                        None
                    };
                    if name.is_some() && content.is_some() {
                        let nt = Metatag {
                            name: name.unwrap(),
                            content: content.unwrap(),
                        };
                        metatags.push(nt);
                    }
                }
            }
        }
        if metatags.is_empty() {
            None
        } else {
            Some(metatags)
        }
    }
}

#[derive(Debug)]
pub struct Rss {
    pub href: String,
    pub title: Option<String>,
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
    pub rss: Option<Vec<Rss>>,
    pub metatags: Option<Vec<Metatag>>,
}

impl PageInfo {
    pub fn from_str(input: &str) -> PageInfo {
        let dom = tl::parse(&input, tl::ParserOptions::default()).unwrap();
        let parser = dom.parser();
        let html_parser = HtmlParser::new(&dom, parser);
        let title = html_parser.get_title();
        let description = html_parser.get_description();
        let canonical = html_parser.get_canonical();
        let language = html_parser.get_language();
        let rss = html_parser.get_rss();
        let metatags = html_parser.get_metatags();
        PageInfo {
            title,
            description,
            canonical,
            language,
            rss,
            metatags,
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
        println!("{:?}", page_info);
        assert_eq!(
            page_info.title,
            Some(
                "İngiliz Dizileri: ABD Yapımlarından Sıkılanlara 35 İngiliz Dizisi - CEOtudent"
                    .to_string()
            )
        );
        assert_eq!(page_info.language, Some("tr-TR".to_string()));
        assert_eq!(
            page_info.description,
            Some(
                "ABD yapımı dizilerden sıkıldıysanız doğru yerdesiniz. İngiliz dizileri listemizle Birleşik Krallığa doğru heyecanlı bir yolculuğa çıkıyoruz."
                    .to_string()
            )
        );
        assert_eq!(
            page_info.canonical,
            Some("https://ceotudent.com/ingiliz-dizileri".to_string())
        );
        assert_eq!(
            page_info.rss.unwrap().first().unwrap().href,
            "rss.xml".to_string()
        );
    }
}
