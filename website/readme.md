develop by installing
```bash
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli
```
then running `trunk serve`.

Build with `trunk build --release --public-url "./"`
and move the dist/ contents into the pages branch.

  - https://github.com/thedodd/trunk

TODO it's in a quite messy state. Convert to sauron and clean up
