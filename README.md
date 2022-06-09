# TinyMD, a tiny Markdown compiler

This is the TinyMD compiler that you build in the free online
book [Getting Started with Rust](https://jesselawson.github.io/getting-started-with-rust). It understands basic first-level 
headings and paragraphs only, and leaves room for improvement. 

## Usage

1. Clone this repo:

```
$ git clone https://www.github.com/jesselawson/tinymd.git
```

2. Build from source:

```
$ cd tinymd && cargo build
```

3. Run with the included Markdown file (or your own):

```
$ cargo run hello.md
```

## Contributing

This repository must match the code content in [Getting Started with Rust](https://jesselawson.github.io/getting-started-with-rust), 
so code parity between this repo and the book is paramount. 

Any PR to this repo will require a PR to the book, and possibly 
a content update.
