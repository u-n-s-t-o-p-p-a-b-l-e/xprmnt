```
rustc -L vendor url_shortener.rs

```

```
./url_shortener "https://www.example.com"
```

Shortened URL: A1b2C3

Retrieve the original URL:

```
./url_shortener retrieve "A1b2C3"
```

This will output the original URL:

```
Original URL: https://www.example.com
```

ps: need to download rand and place it in 'vendor' directory in this project.
