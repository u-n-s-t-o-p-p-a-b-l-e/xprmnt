Code for high performance in Rust.
<br>


Run quicksort.rs with:

```
rustc -C opt-level=3 quicksort.rs -o quicksort && ./quicksort
```

Run Matrix_multiplication.rs with:
```
rustc -C opt-level=3 matrix_multiplication.rs -o matrix_multiplication && ./matrix_multiplication
```

For http_server.rs:

```
rustc -C opt-level=3 http_server.rs -o http_server
```

Run it and open browser and navigate to `http://127.0.0.7878` to see the "Hi world!" response.
```
./http_server
```
