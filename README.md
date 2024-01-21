# colorhash

Convert a string to a color representation consistently by using a sha256 hash.

Based on [color-hash](https://github.com/zenozeng/color-hash/) for JS. Doesn't use a custom hashing function, it's always sha256.

To try out how it works:

```
rustup target add wasm32-unknown-unknown
cargo install trunk
cd demo-page
trunk serve
```

You should see a result like this:

![Demo page](https://raw.githubusercontent.com/berkus/colorhash/main/colorhash-demo.png)
