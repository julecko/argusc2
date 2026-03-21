CREATE TABLE upload_tickets (
    id                  INT             NOT NULL AUTO_INCREMENT PRIMARY KEY,

    -- identity
    token               CHAR(64)        NOT NULL UNIQUE,     -- SHA-256 random token, used in URL
    device_id           INT             NOT NULL,            -- which device this ticket was issued to

    -- limits
    max_files           INT UNSIGNED    NOT NULL DEFAULT 1,  -- how many files can be uploaded with this ticket
    max_size_bytes      BIGINT UNSIGNED NOT NULL,            -- max size per single file

    -- usage tracking
    files_uploaded      INT UNSIGNED    NOT NULL DEFAULT 0,
    bytes_uploaded      BIGINT UNSIGNED NOT NULL DEFAULT 0,

    -- state
    status              ENUM('active', 'exhausted', 'expired', 'revoked') NOT NULL DEFAULT 'active',
    expires_at          TIMESTAMP       NOT NULL,
    created_at          TIMESTAMP       NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMP       NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    CONSTRAINT fk_ticket_device
        FOREIGN KEY (device_id)
        REFERENCES devices(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
