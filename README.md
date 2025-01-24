Inspired by python logging library `loguru`.

```rust
fn main() {
    log::log(log::Level::Critical, "Hello, world!");
    log::critical("Hello, world!");
    log::error("Hello, world!");
    log::warning("Hello, world!");
    log::success("Hello, world!");
    log::info("Hello, world!");
    log::debug("Hello, world!");
    log::trace("Hello, world!");
}
```
