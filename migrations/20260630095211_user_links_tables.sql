-- Add migration script here
CREATE TABLE users (
    id UUID PRIMARY KEY,
    password_hash VARCHAR(255) NOT NULL,
    username VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()

);

CREATE TABLE links (
    id UUID PRIMARY KEY,
    url TEXT NOT NULL,
    short_url TEXT UNIQUE NOT NULL,
    account UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);