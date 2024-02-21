CREATE TABLE IF NOT EXISTS user_sessions
(
    id                      uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id                 uuid NOT NULL,
    session_token_p1 text   NOT NULL,
    session_token_p2 text   NOT NULL,
    created_at              BIGINT NOT NULL,
    expires_at              BIGINT NOT NULL
);
