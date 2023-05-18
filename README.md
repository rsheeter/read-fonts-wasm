# use-skrifa

Do [basic things](src/main.rs) with skrifa and see how big the resulting binary is.

There are several profiles, see `Cargo.toml`.

```shell
profiles=(release release-strip release-lto release-lto-strip release-lto-strip-abort)
rm -rf target/
for profile in ${profiles[@]}; do
    cargo build --profile "${profile}" ;
done
for profile in ${profiles[@]}; do
    ls -l "target/${profile}/use_skrifa" | cut -d " " -f 5,9| awk '{print "| "$2" | "$1" |"}'
done
```

Current results:

| profile | size (bytes) |
| --- | --- |
| target/release/use_skrifa | 4263336 |
| target/release-strip/use_skrifa | 317824 |
| target/release-lto/use_skrifa | 1782600 |
| target/release-lto-strip/use_skrifa | 285056 |
| target/release-lto-strip-abort/use_skrifa | 280960 |

https://github.com/johnthagen/min-sized-rust suggests options beyond what I have tried here.

## panic_immediate_abort

Using `panic_immediate_abort` combined with `build-std` seems to provide a substantial reduction in file size (down to 68680 on macOS).

Instructions are as follows:

Setup:
```
# Install nightly toolchain and source
$ rustup toolchain install nightly
$ rustup component add rust-src --toolchain nightly

# Find your target triple
$ rustc -vV
...
host: aarch64-apple-darwin
```

Compiling (using `--target=<your-target-triple>`):
```
$ cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --profile=release-lto-strip-abort --target aarch64-apple-darwin
```

See the results:
```
$ ls -l target/aarch64-apple-darwin/release-lto-strip-abort/use_skrifa 
-rwxr-xr-x  1 user  staff  68680 May 18 14:51 target/aarch64-apple-darwin/release-lto-strip-abort/use_skrifa
```