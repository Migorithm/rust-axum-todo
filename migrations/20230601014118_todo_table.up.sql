-- Add down migration script here
CREATE TABLE IF NOT EXISTS todo_list(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    text TEXT NOT NULL,
    completed BOOLEAN NOT NULL
);