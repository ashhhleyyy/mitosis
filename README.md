# Mitosis

Small service used to access minigame translations from the website.

## Configuration
```toml
# config.toml

### SOURCES
# All these options can be omitted to disable them

# Directory containing mod JARs to load translations from
mods_dir = "/srv/server/mods/"
# Directory containing datapacks to load translations from
datapacks_dir = "/srv/server/datapacks/"

### TRANSLATION

# The default locale code to fall back to
# if translations are not available in other languages
fallback_locale = "en_us"

### HTTP SERVER

# The port for the HTTP server to listen on
listen_port = 4040
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
