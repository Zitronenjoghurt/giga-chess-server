CREATE TABLE IF NOT EXISTS invite_codes
(
    id         UUID PRIMARY KEY,
    used       BOOLEAN     NOT NULL DEFAULT FALSE,
    comment    VARCHAR              DEFAULT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS users
(
    id             UUID PRIMARY KEY,
    name           VARCHAR UNIQUE NOT NULL,
    invite_code_id UUID           NOT NULL REFERENCES invite_codes (id),
    password_hash  VARCHAR        NOT NULL,
    created_at     TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    updated_at     TIMESTAMPTZ    NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS rooms
(
    id         UUID PRIMARY KEY,
    name       VARCHAR,
    public     BOOLEAN     NOT NULL,
    created_by UUID        NOT NULL REFERENCES users (id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);