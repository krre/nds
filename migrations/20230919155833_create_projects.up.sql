CREATE TABLE IF NOT EXISTS projects (
    id serial PRIMARY KEY,
    name TEXT NOT NULL,
    template smallint NOT NULL,
    description TEXT,
    created_ts timestamptz NOT NULL DEFAULT now(),
    updated_ts timestamptz NOT NULL DEFAULT now()
);
