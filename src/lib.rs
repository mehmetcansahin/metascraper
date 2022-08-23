struct HtmlParser<'a> {
    dom: &'a tl::VDom<'a>,
    parser: &'a tl::Parser<'a>,
}

impl HtmlParser<'_> {
    fn new<'a>(dom: &'a tl::VDom<'a>, parser: &'a tl::Parser<'a>) -> HtmlParser {
        HtmlParser { dom, parser }
    }

    fn get_element_intext(&self, selector: &str) -> Option<String> {
        let node_option = self.dom.query_selector(selector).unwrap().next();
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
        let node_option = self.dom.query_selector(selector).unwrap().next();
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
}

#[derive(Debug)]
pub struct PageInfo {
    pub title: Option<String>,
    pub description: Option<String>,
}

impl PageInfo {
    pub fn from_str(input: &str) -> PageInfo {
        let dom = tl::parse(&input, tl::ParserOptions::default()).unwrap();
        let parser = dom.parser();
        let html_parser = HtmlParser::new(&dom, parser);
        let title = html_parser.get_element_intext("title");
        let description = html_parser.get_element_attr("meta[name=description]", "content");
        PageInfo { title, description }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let html = include_str!("test.html");
        let pageinfo = PageInfo::from_str(html);
        println!("{:?}", pageinfo);
    }
}
