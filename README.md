`C:\Users\<User>\.cargo\config.toml`

``` toml
[target.x86_64-pc-windows-msvc]
rustflags = [
  "-C", "link-arg=/DEBUG:NONE",
  "-C", "target-feature=+crt-static",
]
[source.crates-io]
replace-with = 'aliyun'
[source.aliyun]
registry = "sparse+https://mirrors.aliyun.com/crates.io-index/"
```

cargo add chardet encoding_rs windows-sys
