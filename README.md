# MetaScraper

[![Rust](https://github.com/mehmetcansahin/metascraper/actions/workflows/rust.yml/badge.svg)](https://github.com/mehmetcansahin/metascraper/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/metascraper.svg)](https://crates.io/crates/metascraper)
[![Released API docs](https://docs.rs/metascraper/badge.svg)](https://docs.rs/metascraper)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

MetaScraper is a extracts metadata information of a website. MetaScraper uses [tl](https://github.com/y21/tl) as its html parser. This choice was made because tl was the fastest in benchmark tests. For more information, visit the [parse_query_bench](https://github.com/mehmetcansahin/parse_query_bench) GitHub page.

## Usage

```rust
let input = include_str!("test.html");
let metascraper = MetaScraper::parse(input).unwrap();
let metadata = metascraper.metadata();
println!("{:?}", metadata);
// MetaData {
//     title: Some("Title"),
//     description: Some("Description"),
//     canonical: Some("https://mehmetcan.sahin.dev"),
//     language: Some("en"),
//     rss: Some("rss.xml"),
//     metatags: Some([
//         Metatag { name: "X-UA-Compatible", content: "IE=edge" },
//         Metatag { name: "viewport", content: "width=device-width, initial-scale=1.0" },
//         Metatag { name: "description", content: "Description" }
//         ])
// }
```

## License

MetaScraper is licensed under the MIT License.
