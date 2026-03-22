CREATE TABLE programs (
    id                INT                                        NOT NULL AUTO_INCREMENT PRIMARY KEY,
    type_id           INT                                        NOT NULL,
    uploaded_by       INT                                        NULL,
    name              VARCHAR(255)                               NOT NULL,
    original_name     VARCHAR(255)                               NOT NULL,
    version           VARCHAR(50)                                NOT NULL,
    os                ENUM('windows', 'linux', 'android', 'mac') NOT NULL,
    storage_path      VARCHAR(512)                               NOT NULL,
    filesize          BIGINT                                     NOT NULL, -- in bytes
    file_hash         CHAR(64)                                   NOT NULL, -- e.g., SHA-256
    ws_key            CHAR(64)                                   NOT NULL UNIQUE, -- e.g., SHA-256
    description       TEXT                                       NULL,
    downloads         INT                                        NOT NULL DEFAULT 0,
    allowed_downloads INT                                        NOT NULL DEFAULT 0,
    created_at        TIMESTAMP                                  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at        TIMESTAMP                                  NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    CONSTRAINT fk_program_type
        FOREIGN KEY (type_id)
        REFERENCES program_types(id)
        ON DELETE RESTRICT
        ON UPDATE CASCADE,

    CONSTRAINT fk_program_uploader
        FOREIGN KEY (uploaded_by)
        REFERENCES admins(id)
        ON DELETE SET NULL
        ON UPDATE CASCADE
);
