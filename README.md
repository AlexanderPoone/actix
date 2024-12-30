<!--New!!! Your mom resorts to old ways - stability (...), Austria (which is old), Chinese geomancy (which is old), and the Pope (...) simutaneously.-->
Forked this repo to learn actix-web.

What I've learned: ***actix + Vue is a popular and perfect combo***

There's also a [chat sample](websockets/chat/static/index.html) parallel to Rocket's! `const socket = new WebSocket(wsUri);` vs `const events = new EventSource(uri); events.addEventListener("message", (ev) => { ... }));` 'Exponential backoff reconnect'

## Usage
* [actix_cors::Cors::default()](cors/backend/src/main.rs) `move || { App::new().wrap( ... ) }` ***CORS is also a [Vue ^3.3.4](https://github.com/AlexanderPoone/actix/blob/master/cors/frontend/package.json) sample !!! This is also a 'log in' sample.***
* [deadpool_postgres::Client.prepare(&_stmt)](databases/postgres/src/db.rs) `-> Result<Vec<User>, MyError>`
* `client.query(&stmt, &[&user_info.email, &user_info.first_name, &user_info.last_name, &user_info.username,],)`
* [pg_mapper](databases/postgres/src/models.rs)

## Community Showcase
Many of these repos use [SQLX](https://github.com/launchbadge/sqlx) for Postgres, e.g., `sqlx::postgres::PgPoolOptions` (Pg for Postgres):
```rust
use sqlx::postgres::PgPoolOptions;

let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect("postgres://postgres:password@localhost/test").await?;        // Alex: Get this from ENV

// Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL/MariaDB)
let row: (i64,) = sqlx::query_as("SELECT $1")
    .bind(150_i64)                                                         // Alex: This is even nicer than SQLAlchemy in Python
    .fetch_one(&pool).await?;

assert_eq!(row.0, 150);

Ok(())
```
- [Actix-Web Shuttle Template](https://github.com/sentinel1909/shuttle-templat-actix/blob/master/Cargo.toml): A somewhat opinionated template for getting started with an Actix Web API and hosting it on Shuttle.
- [Atomic-Server](https://github.com/joepio/atomic-data-rust/blob/master/Cargo.toml): Graph database with a dynamic schema, authorization and full text search.
- ***[Barricade](https://github.com/purton-tech/barricade/blob/master/Cargo.toml): Quickly add user registration and logon to any application.***
  - Interesting. Comes with hCaptcha too. 'Works well as a Kubernetes side car.'
  - `use sqlx::PgPool;` as well. Postgres + Actix = nice combo
- [binserve](https://github.com/mufeedvh/binserve/blob/master/Cargo.toml): A fast, secure, and easy to set up static web server written on top of Actix Web with routing, templating, and various other features.
- ***[Bloom](https://github.com/skerkour/bloom-legacy/blob/master/Cargo.toml): The simplest way to de-Google your life and business: Inbox, Calendar, Files, Contacts & much more.***
  - [It uses Vuetify 2 (EOL) + Axios](https://github.com/skerkour/bloom-legacy/blob/fa91a0fcfb779a5657cbbbfaf9ea305e79570122/webapp/package.json)
- [Contile](https://github.com/mozilla-services/contile/blob/master/Cargo.toml): The back-end server for the Mozilla Tile Service (MTS).
- [Dalted](https://github.com/carrascomj/dalted/blob/master/Cargo.toml): Simple webapp that showcases the integration of [image-rs](https://github.com/image-rs/image) with Actix Web for color blindness simulations.
- [Four in a Row - Server](https://github.com/ffactory-ofcl/fourinarow-server/blob/master/Cargo.toml): An online version of the popular game four in a row, written in Rust on the server side and Flutter + Dart on the client.
- [gcs-proxy](https://github.com/guaychou/gcs-proxy/blob/master/Cargo.toml): A Google Cloud Storage download proxy
- [GitArena](https://github.com/mellowagain/gitarena/blob/master/Cargo.toml): Software development platform with built-in VCS, issue tracking and code review.
  - vs. GitLab self-hosted, and Gitea (written in Go, 45k stars)
- [hyperswitch](https://github.com/juspay/hyperswitch/blob/master/Cargo.toml): An Open Source Financial Switch to make payment open, fast, secure and reliable.
- ***[Labrinth](https://github.com/modrinth/labrinth/blob/master/Cargo.toml): Rust-based backend to serve the `modrinth` API.***
- [lemmy](https://github.com/dessalines/lemmy/blob/master/Cargo.toml): A federated alternative to reddit in Rust.
- [Martin](https://github.com/maplibre/martin/blob/master/Cargo.toml): Blazing fast and lightweight PostGIS, MBtiles and PMtiles tile server.
- ***[mCaptcha](https://github.com/mCaptcha/mCaptcha/blob/master/Cargo.toml): Proof of work based, privacy focused, libre CAPTCHA system. Crates used: `actix-web`, `sqlx`, `redis`, and `lettre`.***
- [MeiliSearch](https://github.com/meilisearch/MeiliSearch/blob/master/Cargo.toml): Fast, Relevant and Typo-Tolerant Search Engine. Open source alternative to Algolia.
- [Merino](https://github.com/mozilla-services/merino/blob/master/Cargo.toml): Web service for Firefox Suggest.
- [Nitro Repo](https://github.com/wherkamp/nitro_repo/blob/master/Cargo.toml): An open source ***artifact (build output)*** manager. ***Rust back-end and Vue front-end.***
- [Operator](https://github.com/mkantor/operator/blob/master/Cargo.toml): A web server for static and dynamic content.
- [pict-rs](https://git.asonix.dog/asonix/pict-rs/blob/master/Cargo.toml): An image host API service
- [RCOS Telescope](https://github.com/rcos/Telescope/blob/master/Cargo.toml): The RCOS (Ren,sse`lae,r /rɛnsəˈlɪər/ Polytechnic Institute) website and Discord bot.
- [Roseline](https://github.com/DoumanAsh/roseline.rs/blob/master/Cargo.toml): A personal web site and discord & ***IRC (Internet relay chat)*** bot to access simple SQLite database. Demonstrates usage of various Actix and Actix Web concepts.
- [rustus](https://github.com/s3rius/rustus/blob/master/Cargo.toml): A ***[TUS - the open protocol for resumable file uploads](https://github.com/tus/tusd)*** implementation that helps you handle file uploads
- [rustypaste](https://github.com/orhun/rustypaste/blob/master/Cargo.toml): A minimal file upload/pastebin service
- [tokei.rs](https://github.com/XAMPPRocky/tokei_rs/blob/master/Cargo.toml): The tokei.rs server code.
- [trieve](https://github.com/devflowinc/trieve/blob/master/Cargo.toml): All-in-one infrastructure for building search, recommendations, and ***Retrieval Augmented Generation (RAG)***.
- [Triox](https://github.com/Trioxidation/Triox/blob/master/Cargo.toml): A free file hosting server that focuses on speed, reliability and security.
- [WebThings Registration Server](https://github.com/WebThingsIO/registration_server/blob/master/Cargo.toml): Exposes an HTTP API that lets you register a WebThings Gateway for tunneling support
- [Zero2prod](https://github.com/LukeMathWalker/zero-to-production/blob/master/Cargo.toml): Source code of zero to production book [zero2prod.com](https://www.zero2prod.com). Paid book but some of the chapters are available online for free. The book compares and explains the chosen technologies, like Actix Web and SQLx.

## Community Articles, Example Apps, Starters & Boilerplate Projects

- [Actix and SQLx User CRUD for MySQL](https://github.com/jamesjmeyer210/actix_sqlx_mysql_user_crud): A User CRUD showcasing MySQL database interaction with full integration test coverage, designed to fit comfortably in a system of micro-services.
- [Actix Server Authentication with JWT and MongoDB](https://github.com/emreyalvac/actix-web-jwt/): An implementation of JWT in Actix.
- [Blog with markdown rendering](https://github.com/gemini-15/blog-engine): Blog example built with Actix Web, diesel (with Postgres) and r2d2 rendering articles in markdown with metadata and a front-end with React.
- [Canduma](https://github.com/clifinger/canduma): Rust authentication server boilerplate
- [Complete Actix 2.x REST Server](https://github.com/ddimaria/rust-actix-example): Actix 2.x HTTP Server featuring multi-database support, auth/JWTs, caching, static files, app state, tests, coverage, and docker.
- [create-rust-app](https://github.com/Wulf/create-rust-app): Set up a modern Rust + React web app by running one command.
- [Fullstack-Rust](https://github.com/vascokk/fullstack-rust): A Full Stack Rust application (Connect5 game) with Actix Web, Yew, Bulma CSS and Diesel.
- [Mozilla Services Skeleton App](https://github.com/mozilla-services/skeleton)
- [planters_cycle](https://github.com/grimm-integrations/planters_cycle): Another boilerplate fullstack application with identity system, prisma and NextJs.
- [Production-Grade Logging in Rust Applications](https://medium.com/better-programming/production-grade-logging-in-rust-applications-2c7fffd108a6): An article showcasing the use of [tracing](https://github.com/tokio-rs/tracing) in an Actix application
- [rayspace.dev](https://github.com/ryspc/rayspace.dev): Minimalist dev portfolio and blog implemented as a Rust-powered SPA, featuring GitHub OAuth, session management, static file serving, API endpoints, and SQLx integration.
- [RealWorld Example App](https://github.com/fairingrey/actix-realworld-example-app): Implementation of the RealWorld backend API spec in Actix.
- [Rust, Actix Web & Heroku](https://github.com/emk/rust-buildpack-example-actix): A Heroku buildpack example for Actix Web.
- [Rust, Angular, PostgreSQL and JWT Security](https://github.com/stav121/actix-angular-project-template): Boilerplate project that implements an Angular + Actix Web application with login and registration pages, that is pre-dockerized.
- [Rust, Docker & GraphQL](https://github.com/jayy-lmao/rust-graphql-docker): An example of using Dataloaders, context, and a minimal docker container.
- [webapp.rs](https://github.com/saschagrunert/webapp.rs): A web application completely written in Rust.
