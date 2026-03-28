CREATE TABLE events (
    id         INT          NOT NULL AUTO_INCREMENT PRIMARY KEY,
    device_id  INT          NOT NULL,
    level      ENUM('low', 'medium', 'high') NOT NULL DEFAULT 'low',
    message    TEXT         NOT NULL,
    created_at TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT fk_event_device
        FOREIGN KEY (device_id) REFERENCES devices(id)
        ON DELETE CASCADE
);
