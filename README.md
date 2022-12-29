# Resella

## Setup

### Install Dependencies

    cargo update

### Prepare Environment

Copy the example configuration at `.env.example` to `.env` and adjust it into your local config values

## Migrations

The migration tools is using `diesel_cli`, so you'll need to install it first by

    cargo install diesel_cli

### View Help

    diesel migration

### Create new Migration

    diesel migration generate <your-migration-name>

### Run Migration

    diesel migration run

### Redo Migration

    diesel migration redo

