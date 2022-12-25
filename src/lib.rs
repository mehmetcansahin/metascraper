use tl::{ParseError, ParserOptions, VDom};

#[derive(Debug)]
pub struct Metatag {
    pub name: String,
    pub content: String,
}

#[derive(Debug)]
pub struct MetaData {
    pub title: Option<String>,
    pub description: Option<String>,
    pub canonical: Option<String>,
    pub language: Option<String>,
    pub rss: Option<String>,
    pub metatags: Option<Vec<Metatag>>,
}

pub struct MetaScraper<'a> {
    dom: VDom<'a>,
}

impl MetaScraper<'_> {
    pub fn parse(input: &str) -> Result<MetaScraper, ParseError> {
        match tl::parse(input, ParserOptions::default()) {
            Ok(dom) => Ok(MetaScraper { dom }),
            Err(err) => Err(err),
        }
    }

    pub fn metatags(&self) -> Option<Vec<Metatag>> {
        let mut metatags: Vec<Metatag> = Vec::new();
        let query_sellector_iter = self.dom.query_selector(r#"meta"#)?;
        for node_handle in query_sellector_iter {
            let node = node_handle.get(self.dom.parser())?;
            if let Some(tag) = node.as_tag() {
                let name = tag
                    .attributes()
                    .get("name")
                    .or_else(|| tag.attributes().get("property"))
                    .or_else(|| tag.attributes().get("itemprop"))
                    .or_else(|| tag.attributes().get("http-equiv"))
                    .flatten()
                    .map(|x| x.as_utf8_str().to_string());

                let content = tag
                    .attributes()
                    .get("content")
                    .or_else(|| tag.attributes().get("description"))
                    .flatten()
                    .map(|x| x.as_utf8_str().to_string());

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

    pub fn get_intext(&self, selector: &str) -> Option<String> {
        self.dom
            .query_selector(selector)
            .and_then(|mut iter| iter.next())
            .and_then(|node_handle| node_handle.get(self.dom.parser()))
            .map(|node| node.inner_text(self.dom.parser()).to_string())
    }

    pub fn get_attribute(&self, selector: &str, attr: &str) -> Option<String> {
        self.dom
            .query_selector(selector)
            .and_then(|mut iter| iter.next())
            .and_then(|node_handle| node_handle.get(self.dom.parser()))
            .and_then(|node| node.as_tag())
            .and_then(|html_tag| html_tag.attributes().get(attr).flatten())
            .map(|bytes| bytes.as_utf8_str().to_string())
    }

    pub fn rss(&self) -> Option<String> {
        self.get_attribute(r#"link[type*=rss]"#, "href")
            .or_else(|| self.get_attribute("meta[property*=feed]", "href"))
            .or_else(|| self.get_attribute("meta[property*=atom]", "href"))
    }

    pub fn title(&self) -> Option<String> {
        self.get_intext("title")
            .or_else(|| self.get_attribute("meta[property*=title]", "content"))
            .or_else(|| self.get_intext(".post-title"))
            .or_else(|| self.get_intext(".entry-title"))
            .or_else(|| self.get_intext("h1[class*=title] a"))
            .or_else(|| self.get_intext("h1[class*=title]"))
    }

    pub fn description(&self) -> Option<String> {
        self.get_attribute(r#"meta[name*=description]"#, "content")
            .or_else(|| self.get_attribute("meta[property*=description]", "content"))
            .or_else(|| self.get_attribute("meta[itemprop*=description]", "content"))
            .or_else(|| self.get_attribute("meta[description]", "description"))
    }

    pub fn canonical(&self) -> Option<String> {
        self.get_attribute("link[rel=canonical]", "href")
            .or_else(|| self.get_attribute("meta[property*=url]", "content"))
            .or_else(|| self.get_attribute("meta[name*=url]", "content"))
            .or_else(|| self.get_attribute("link[rel=alternate][hreflang*=default]", "href"))
    }

    pub fn language(&self) -> Option<String> {
        self.get_attribute("html", "lang")
            .or_else(|| self.get_attribute("meta[itemprop=inLanguage]", "content"))
            .or_else(|| self.get_attribute("meta[property*=locale]", "content"))
    }

    pub fn metadata(&self) -> MetaData {
        MetaData {
            title: self.title(),
            description: self.description(),
            canonical: self.canonical(),
            language: self.language(),
            rss: self.rss(),
            metatags: self.metatags(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::MetaScraper;

    #[test]
    fn test_page() {
        let input = include_str!("test.html");
        let metascraper = MetaScraper::parse(input).unwrap();
        let metadata = metascraper.metadata();
        assert_eq!(metadata.title, Some("Title".to_string()));
        assert_eq!(metadata.language, Some("en".to_string()));
        assert_eq!(metadata.description, Some("Description".to_string()));
        assert_eq!(
            metadata.canonical,
            Some("https://mehmetcan.sahin.dev".to_string())
        );
        assert_eq!(metadata.rss, Some("rss.xml".to_string()));
    }
}
