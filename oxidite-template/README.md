# oxidite-template

Template engine for server-side rendering in Oxidite.

## Installation

```toml
[dependencies]
oxidite-template = "0.1"
```

## Usage

```rust
use oxidite_template::*;

let engine = TemplateEngine::new("templates");

// Render template
let html = engine.render("index.html", context! {
    title: "Home",
    user: user
}).await?;

Ok(Response::html(html))
```

## License

MIT
