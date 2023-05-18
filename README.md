# use-skrifa

Do basic things with skrifa and see how big the resulting binary is.

There are several profiles, see `Cargo.toml`.

```shell
profiles=(release release-lto release-lto-abort)
rm -rf target/
for profile in ${profiles[@]}; do
    cargo build --profile "${profile}" ;
done
for profile in ${profiles[@]}; do
    ls -l "target/${profile}/use_skrifa"
done
```
