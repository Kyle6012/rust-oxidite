# oxidite-mail

Email sending for Oxidite with SMTP support.

## Installation

```toml
[dependencies]
oxidite-mail = "0.1"
```

## Usage

```rust
use oxidite_mail::*;

let mailer = Mailer::new("smtp.gmail.com", 587)
    .auth("user@gmail.com", "password");

mailer.send(
    Email::new()
        .to("recipient@example.com")
        .subject("Hello")
        .body("Email content")
).await?;
```

## License

MIT
