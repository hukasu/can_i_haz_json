# ![Can i haz json](assets/can_i_haz_json.svg)

Minimalist, `#![no_std]`, and no dependencies JSON reader.

## Usage

Adding the dependency to your `Cargo.toml`
```toml
[dependencies]
can_i_haz_json = { git = "https://github.com/hukasu/can_i_haz_json" }
```

Then you can parse your [`JSON`] with
```rust
use std::borrow::Cow;

use can_i_haz_json::Json;

let content = "{\"name\": \"John\"}";
let json = Json::parse(Cow::Borrowed(content));
```

## Resources

The JSON specification can be found on the following links.
* https://www.ietf.org/ietf-ftp/rfc/std/std90.txt

This projects uses the test suite produced by Nicolas Seriot for his
[Parsing JSON is a minefield](https://seriot.ch/security/parsing_json.html)
article. The article and repository for the test suite can be found below.
* https://seriot.ch/security/parsing_json.html
* https://github.com/nst/JSONTestSuite
