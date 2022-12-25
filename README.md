# MetaScraper

## Usage

Add the following line to your Cargo.toml file:

```toml
metascraper = "0.1.0"
```

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
