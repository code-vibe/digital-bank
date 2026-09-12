# DigitalBank

## Local development

Start PostgreSQL:
```bash
docker compose up -d
sqlx migrate run
cargo run -p api
curl http://127.0.0.1:8080/healthz
```

## Docs map

```text
docs/
├── architecture/   # target design, roadmap
├── adr/            # Architecture Decision Records
├── domain/         # Banking domain deep-dives 
├── api/            # API contracts 
├── database/       # Schema / migrations notes
├── security/       # Threat model and decisions
├── operations/     # Runbooks, deploy, observability
└── learning/       # Your Rust / systems journal
```


## Status

