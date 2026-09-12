# DigitalBank

Production-minded digital banking platform in Rust (open-source stack).

## Local development

```bash
cp .env.example .env
docker compose up -d
sqlx migrate run
cargo run -p api
curl http://127.0.0.1:8080/healthz
curl http://127.0.0.1:8080/readyz
```

## Docs

Engineering docs live under `docs/` (gitignored until you choose to publish). Start at `docs/README.md`.

## Status

- Stage 0 foundations in progress
- Architecture contract locked (incl. backoffice maker-checker; Redis 6 / NATS 7 / rails 8)
- Frontends: agent-built later — brief in `docs/frontend/`
- Learning current: connection pooling; later queue includes maker-checker, recon, fraud ops
