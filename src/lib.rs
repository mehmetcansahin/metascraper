//! MetaScraper
//!
//! `metascraper` is on [Crates.io][crate] and [GitHub][github].
//!
//! [crate]: https://crates.io/crates/metascraper
//! [github]: https://github.com/mehmetcansahin/metascraper
//!
//! # Examples
//!
//! ## Parsing a document
//!
//! ```
//! use metascraper::MetaScraper;
//!
//! let metascraper = MetaScraper::parse(input).unwrap();
//! let metadata = metascraper.metadata();
//! ```

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
    pub image: Option<String>,
    pub amp: Option<String>,
    pub author: Option<String>,
    pub date: Option<String>,
    pub metatags: Option<Vec<Metatag>>,
}

pub struct MetaScraper<'a> {
    dom: VDom<'a>,
}

impl MetaScraper<'_> {
    /// Parse input
    pub fn parse(input: &str) -> Result<MetaScraper, ParseError> {
        match tl::parse(input, ParserOptions::default()) {
            Ok(dom) => Ok(MetaScraper { dom }),
            Err(err) => Err(err),
        }
    }

    /// Returns the inner text of the given selector.
    pub fn inner_text(&self, selector: &str) -> Option<String> {
        self.dom
            .query_selector(selector)
            .and_then(|mut iter| iter.next())
            .and_then(|node_handle| node_handle.get(self.dom.parser()))
            .map(|node| node.inner_text(self.dom.parser()).to_string())
    }

    /// Returns the value of the given attribute of the given selector.
    pub fn attribute(&self, selector: &str, attr: &str) -> Option<String> {
        self.dom
            .query_selector(selector)
            .and_then(|mut iter| iter.next())
            .and_then(|node_handle| node_handle.get(self.dom.parser()))
            .and_then(|node| node.as_tag())
            .and_then(|html_tag| html_tag.attributes().get(attr).flatten())
            .map(|bytes| bytes.as_utf8_str().to_string())
    }

    /// Metatags return in vector.
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

    /// Returns the rss
    pub fn rss(&self) -> Option<String> {
        self.attribute(r#"link[type*=rss]"#, "href")
            .or_else(|| self.attribute("meta[property*=feed]", "href"))
            .or_else(|| self.attribute("meta[property*=atom]", "href"))
    }

    /// Returns the title
    pub fn title(&self) -> Option<String> {
        self.inner_text("title")
            .or_else(|| self.attribute("meta[property*=title]", "content"))
            .or_else(|| self.inner_text(".post-title"))
            .or_else(|| self.inner_text(".entry-title"))
            .or_else(|| self.inner_text("h1[class*=title] a"))
            .or_else(|| self.inner_text("h1[class*=title]"))
    }

    /// Returns the description
    pub fn description(&self) -> Option<String> {
        self.attribute(r#"meta[name*=description]"#, "content")
            .or_else(|| self.attribute("meta[property*=description]", "content"))
            .or_else(|| self.attribute("meta[itemprop*=description]", "content"))
            .or_else(|| self.attribute("meta[description]", "description"))
    }

    /// Returns the canonical
    pub fn canonical(&self) -> Option<String> {
        self.attribute("link[rel=canonical]", "href")
            .or_else(|| self.attribute("meta[property*=url]", "content"))
            .or_else(|| self.attribute("meta[name*=url]", "content"))
            .or_else(|| self.attribute("link[rel=alternate][hreflang*=default]", "href"))
    }

    /// Returns the language
    pub fn language(&self) -> Option<String> {
        self.attribute("html", "lang")
            .or_else(|| self.attribute("meta[itemprop=inLanguage]", "content"))
            .or_else(|| self.attribute("meta[property*=locale]", "content"))
    }

    /// Returns the image
    pub fn image(&self) -> Option<String> {
        self.attribute("meta[property*=image]", "content")
            .or_else(|| self.attribute("meta[name*=image]", "content"))
            .or_else(|| self.attribute("meta[itemprop*=image]", "content"))
            .or_else(|| self.attribute("article img[src]", "src"))
            .or_else(|| self.attribute("#content img[src]", "src"))
            .or_else(|| self.attribute("img[alt*=author]", "src"))
            .or_else(|| self.attribute("img[src]:not([aria-hidden=true])", "src"))
    }

    /// Returns the amp
    pub fn amp(&self) -> Option<String> {
        self.attribute("link[rel=amphtml]", "href")
    }

    /// Returns the author
    pub fn author(&self) -> Option<String> {
        self.attribute("meta[name*=author]", "content")
            .or_else(|| self.attribute("meta[property*=author]", "content"))
            .or_else(|| self.attribute("meta[itemprop*=author]", "content"))
    }

    /// Returns the date
    pub fn date(&self) -> Option<String> {
        self.attribute("meta[property*=updated_time]", "content")
            .or_else(|| self.attribute("meta[property*=modified_time]", "content"))
            .or_else(|| self.attribute("meta[property*=published_time]", "content"))
            .or_else(|| self.attribute("meta[property*=release_date]", "content"))
            .or_else(|| self.attribute("meta[itemprop*=datemodified]", "content"))
            .or_else(|| self.attribute("meta[itemprop*=date]", "datetime"))
            .or_else(|| self.attribute("meta[name*=date]", "content"))
            .or_else(|| self.inner_text(".byline"))
            .or_else(|| self.inner_text(".dateline"))
            .or_else(|| self.inner_text(".date"))
            .or_else(|| self.inner_text("#date"))
            .or_else(|| self.inner_text(".publish"))
            .or_else(|| self.inner_text("#publish"))
            .or_else(|| self.inner_text(".post-timestamp"))
            .or_else(|| self.inner_text("#post-timestamp"))
            .or_else(|| self.inner_text(".time"))
            .or_else(|| self.inner_text("#time"))
    }

    /// Returns the metadata
    pub fn metadata(&self) -> MetaData {
        MetaData {
            title: self.title(),
            description: self.description(),
            canonical: self.canonical(),
            language: self.language(),
            rss: self.rss(),
            metatags: self.metatags(),
            image: self.image(),
            amp: self.amp(),
            author: self.author(),
            date: self.date(),
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
