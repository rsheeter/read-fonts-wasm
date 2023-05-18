# use-skrifa

Do [basic things](src/main.rs) with skrifa and see how big the resulting binary is.

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

Current results:

| profile | size (bytes) |
| --- | --- |
| release | 4263336 |
| release-lto | 1782600 |
| release-lto-abort | 1770016 |
