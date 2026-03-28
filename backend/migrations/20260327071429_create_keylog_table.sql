CREATE TABLE keylog (
    id         INT          NOT NULL AUTO_INCREMENT PRIMARY KEY,
    device_id  INT          NOT NULL,
    data       TEXT         NOT NULL,  -- plaintext keystrokes
    created_at TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT fk_keylog_device
        FOREIGN KEY (device_id) REFERENCES devices(id)
        ON DELETE CASCADE
);
