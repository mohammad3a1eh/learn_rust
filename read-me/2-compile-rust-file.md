### Compile **Rust** file (.rs)

- If the name of the file is `main.rc`, we have:

```shell
rustc main.rs
```

- And the output file will be `main.exe`

<hr>

- If we want the file name to be something else, we use `-o` as follows:

```shell
rustc main.rs -o app.exe
```

- And the output file will be `app.exe`