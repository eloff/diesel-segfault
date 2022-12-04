Steps to reproduce

```
git clone
Edit DATABASE_URL in .env file
diesel migrations run
cargo run
```
(repeat until segfault)

![stacktrace](./segfault.png)