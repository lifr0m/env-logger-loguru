Inspired by python logging library `loguru`.

```rust
fn main() {
    log::log!(log::Level::Critical, "number {}", 1);
    log::critical!("number {}", 1);
    log::error!("number {}", 1);
    log::warning!("number {}", 1);
    log::success!("number {}", 1);
    log::info!("number {}", 1);
    log::debug!("number {}", 1);
    log::trace!("number {}", 1);
}
```
