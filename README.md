# Mitosis

Small service used to access minigame translations from the website.

## Configuration
```toml
# config.toml

### TRANSLATION

# The default locale code to fall back to
# if translations are not available in other languages
fallback_locale = "en_us"

### HTTP SERVER

# The port for the HTTP server to listen on
listen_port = 4040

### SOURCES
[[source]]
type = "datapacks"
path = "datapacks/"

[[source]]
type = "mods"
path = "mods/"

# You can keep adding more [[source]] blocks below if needed
```

### Logging

Mitosis uses [pretty_env_logger](https://crates.io/crates/pretty_env_logger) for logging, which can be configured using the
`RUST_LOG` environment variable, so see their docs for more info.
By default, the variable will be set as `RUST_LOG=info` if not provided.

## HTTP Endpoints

### `GET /translate/<locale>`

```bash
$ curl -H 'Content-Type: application/json' --data '["text.plasmid.game.open.join"]' -X 'GET' -v http://localhost:4040/translate/en_us
...
< HTTP/1.1 200 OK
< content-type: application/json
< content-length: 52
< 
{"text.plasmid.game.open.join":"Click here to join"}
```

### `GET /translate/<locale>/all`

```bash
$ curl -H 'Content-Type: application/json' -X 'GET' -v http://localhost:4040/translate/en_us/all
...
< HTTP/1.1 200 OK
< content-type: application/json
< content-length: 57
< 
{"text.plasmid.game.open.join":"Click here to join", ...}
```

### `GET /locales/all`

```bash
$ curl -H 'Content-Type: application/json' -X 'GET' -v http://localhost:4040/locales/all
...
< HTTP/1.1 200 OK
< content-type: application/json
< content-length: 27
< 
["fr_fr", "et_ee", "en_us"]
```

### `POST /reload`

This endpoint will only actually reload once a minute, to prevent abuse.

```bash
$ curl -v -X 'POST' http://localhost:4040/reload
...
< HTTP/1.1 200 OK
< content-type: application/json
< content-length: 4
< 
"ok"
```

## License

This project is licensed under the [Mozilla Public License 2.0](LICENSE).
