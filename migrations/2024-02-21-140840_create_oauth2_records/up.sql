CREATE TABLE IF NOT EXISTS oauth2_records
(
    id                  uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    csrf_state          VARCHAR(255) NOT NULL,
    pkce_code_verifier  VARCHAR(255) NOT NULL,
    return_url          VARCHAR(255) NOT NULL
);
