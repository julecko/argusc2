CREATE TABLE admins (
    id             INT             NOT NULL AUTO_INCREMENT PRIMARY KEY,
    username       VARCHAR(64)     NOT NULL UNIQUE,
    password_hash  VARCHAR(255)    NOT NULL,
    email          VARCHAR(255)    NOT NULL,
    created_at     TIMESTAMP       NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_login     TIMESTAMP       NULL
);
