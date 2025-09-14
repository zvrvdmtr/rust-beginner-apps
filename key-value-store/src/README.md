## Key-Value storage

### About

### Covered Rust topics

#### Error handling
* If you are asking why you should use [thiserror](https://docs.rs/thiserror/latest/thiserror/) or [anyhow](https://docs.rs/anyhow/latest/anyhow/) crates, read [this](https://momori.dev/posts/rust-error-handling-thiserror-anyhow/#comparison-between-thiserror-and-anyhow).

* If you want more complex explanation about rust error handling strategy then read [this](https://dev-state.com/posts/error_handling/).

* If you want to dig deeper then you need this definitive [guide](https://www.howtocodeit.com/articles/the-definitive-guide-to-rust-error-handling).

#### Web server
I decide to use [Axum](https://github.com/tokio-rs/axum) because it's relatively high-level framework. At this point deep dive to Tokio or Hyper is too much.

* Axum [documentation](https://docs.rs/axum/latest/axum/index.html#high-level-features) the first place you should start.

* Nice [article](https://www.shuttle.dev/blog/2023/12/06/using-axum-rust) from Shuttle which covers all main topics in brief.

* Lots of Axum [examples](https://github.com/tokio-rs/axum/tree/main/examples) from the development team.

* Also you may need middlewares. Axum does not have own middleware module, but fully integrated with [tower](https://docs.rs/tower/latest/tower/) and [tower_http](https://docs.rs/tower-http/latest/tower_http/).

* If you want tracing, then use [tracing](https://docs.rs/tracing/latest/tracing/index.html) and [tracing_subscriber](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/index.html).


