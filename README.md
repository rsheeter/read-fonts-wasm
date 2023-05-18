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
    ls -l "target/${profile}/use_skrifa" | cut -d " " -f 5,9 | awk '{print "| "$1" | "$2" |"}'
done
```

Current results:

| profile | size (bytes) |
| --- | --- |
| 4263336 | target/release/use_skrifa |
| 317824 | target/release-strip/use_skrifa |
| 1782600 | target/release-lto/use_skrifa |
| 285056 | target/release-lto-strip/use_skrifa |
| 280960 | target/release-lto-strip-abort/use_skrifa |
