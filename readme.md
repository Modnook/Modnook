<p align="center">
<img src="./assets/banner.png" alt="fern"/>
<h5 align="center">An open-source community driven mod hosting platform</h3>
</p>
<p align="center">
<a href="./LICENSE"><img alt="License" src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
<a href="https://github.com/modnook/modnook/actions"><img alt="Build status" src="https://github.com/modnook/modnook/actions/workflows/lint.yml/badge.svg"></a>
<a href="https://discord.gg/TODO"><img alt="Discord" src="https://img.shields.io/discord/TODO?label=discord"></a>
</p>

## Roadmap

Modnook is currently in development, if you're interrested in helping out or just want to check out where we're at, first off thank you so much and second; you can find 

## Developing

This repository contains three main components like so:

```
Modnook/
├── server/
│   ├── cmd/
│   ├── migrations/
│   ├── internal/
│   └── justfile
├── site/
│   └── TODO
└── launcher/
    └── TODO
```

| Component  | Description | Status |
|---|---|---|
|`/server`   | Go Backend API and database layer  | DEV |
|`/site`     | Official site for browsing mods  | BLOCKED  |
|`/launcher` | Official launcherx  | BLOCKED |


### Getting Started

#### Prerequesites

- [Go](https://go.dev/dl/) 1.26+
- [Docker](https://docs.docker.com/)
- [Just](https://github.com/casey/just) (optional buildtool)
- [Goose](https://github.com/pressly/goose)
- [pnpm](https://pnpm.io/)

```bash
git clone https://github.com/modnook/modnook.git
```

```
cd server

cp .env.example .env

docker compose up -d

goose -dir migrations postgres "$DATBASE_URL" up

just run-live
```

## Security

If you discover a security vulnerability within our codebase, please **do not** open a public issue. Instead use [GitHub's private vulnerability reporting](https://github.com/modnook/modnook/security/advisories/new) for this repository.

Please include steps to reproduce and the potential impact so we can triage quickly.

## License

Modnook, is licensed under [MIT](./license) you are free to use