A mostly functional interpreter for the [piet] esolang.

Run it with `echo "" | cargo run /path/to/image.format`

Sample programs (most of which work) can be found [here][samples].

Things that it does:
 - Parse programs from most image formats
 - Relatively faithfully execute those programs to spec

Things that I want it to eventually do:
 - Run even with nothing passed to stdin
 - Pre-compute more things about regions
 - Wrap everything up in WASM and build out a frontend that lets you step through execution
 - Have a web-based editor
 - Not be a complete mess

[piet]: https://www.dangermouse.net/esoteric/piet.html
[samples]: https://www.dangermouse.net/esoteric/piet/samples.html
