# What's this?

It's a [TodoMVC](https://todomvc.com/) style app written in [Rust](https://www.rust-lang.org/) with [Yew](https://yew.rs/).

# Screen Recording

![Recording](./recording.gif)

# Run on local

1. Clone this repo
2. Setup Rust + Yew env by this [instruction](https://yew.rs/docs/getting-started/introduction)
3. `trunk serve`
4. Visit http://127.0.0.1:8080 on your browser

# Deploy

```
deploy.sh
```

It's going to deploy the `dist/` to https://yew-todomvc-sunhuawei-stone-playground.vercel.app/.

# Links

1. https://yew.rs/. Yew is a modern Rust framwork for creating multi-threaded frontend web apps using WebAssembly. If you are using React, you will feel at home.
2. https://www.rust-lang.org/. Rust is a language empowering everyone to build reliable and efficient software. It's fast and safe. Its ecosystem is really good. It can be compiled to WASM.
3. https://webassembly.org/. Abbr WASM, can run on web browser.
