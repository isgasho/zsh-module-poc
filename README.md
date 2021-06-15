# zsh-module-poc

Can you make a module for zsh written in Rust? Yes, yes you can and this is a
proof of concept. It's a start but I have no idea how to use Module, things
are definitely safe to use probably, I wouldn't be surprised if there's some
memory leaks here cause of the `ManuallyDrop`s, but that's fine, this is just to
show it is possible. It'd be nice if zsh had good docs, but it doesn't so this
is just what I pieced together after a few hours of work.

Safety: Eh technically
Leaky: Probably
Does it work? Yes.

Want to try it yourself? Make sure you have zsh running.

1. Create a directory
2. `export MODULE_PATH=$MODULE_PATH:/path/to/directory
3. cargo build --release
4. cp target/release/libtest.so /path/to/directory/test.so
5. zmodload test.so
6. You should see "Hello from Rust!" printed to the terminal

Licensing is MIT, it's a PoC I don't really care that much.
