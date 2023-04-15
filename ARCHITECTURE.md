# Architecture

This document describes the software architecture of the application.
The application is split into 'backend' and 'frontend' because it is built with [Tauri](https://tauri.app).

## Backend

The source code for the backend lives in [src-tauri](./src-tauri) and is written in [Rust](https://www.rust-lang.org).

### Recipe Storage

Recipes need to be stored locally. The solution chosen for this project is an embedded relational database. This is supported by simple file storage for binary files like images.

The database needs to implement the data schema outlined in the [PlantUML](https://plantuml.com/) file [schema.puml](./doc/architecture/database/schema.puml).

#### Database

Database connection logic is implemented in the [database module](./src-tauri/src/database.rs).

##### SQLite

[SQLite](https://www.sqlite.org) is used for storing recipe data.

##### SeaORM

The connection to the database is handled via the object-relational manager [SeaORM](https://www.sea-ql.org/SeaORM/).

###### Migrations

SeaORM provides database migration functionality. This is implemented in the [migrator module](./src-tauri/src/migrator.rs).

#### File Storage

Binary files are not stored in the database but separately. Recipe file storage is implemented in the [recipe_file_storage module](./src-tauri/src/recipe_file_storage.rs).

### OCR

Optical character recognition is integrated with [Tesseract](https://github.com/tesseract-ocr/tesseract) via the [tesseract-rs](https://github.com/antimatter15/tesseract-rs) crate.

## Frontend

The source code for the frontend lives in [src](./src) and is transpiled into web languages.

### Svelte

This application uses [Svelte](https://svelte.dev/) as its frontend framework. There is no SSR.
