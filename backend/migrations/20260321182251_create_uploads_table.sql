CREATE TABLE uploads (
    id              INT           NOT NULL AUTO_INCREMENT PRIMARY KEY,
    device_id       INT           NULL,                      -- which device uploaded it
    original_name   VARCHAR(255)  NOT NULL,                  -- filename as sent by implant
    storage_path    VARCHAR(512)  NOT NULL,                  -- where it lives on the server
    mime_type       VARCHAR(127)  NULL,                      -- e.g. 'application/pdf'
    file_hash       CHAR(64)      NOT NULL,                  -- SHA-256 for dedup / integrity
    size_bytes      BIGINT        NOT NULL,
    uploaded_at     TIMESTAMP     NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT fk_upload_device
        FOREIGN KEY (device_id)
        REFERENCES devices(device_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
