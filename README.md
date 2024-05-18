# axum-aide-kata

* https://github.com/tokio-rs/axum
* https://github.com/tamasfe/aide

## Run

```console
just --dotenv-filename .env.local db-only
just --dotenv-filename .env.local local-app
```
```console
curl -i http://0.0.0.0:8000/health
curl -i http://0.0.0.0:8000/openapi.json
```


## Docs

http://0.0.0.0:8000/docs
