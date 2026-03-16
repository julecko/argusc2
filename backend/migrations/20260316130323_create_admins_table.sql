CREATE TABLE admins (
    id          INT             NOT NULL AUTO_INCREMENT PRIMARY KEY,
    username    VARCHAR(64)     NOT NULL UNIQUE,
    password    VARCHAR(255)    NOT NULL,
    created_at  DATETIME        NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_login  DATETIME        NULL
);
