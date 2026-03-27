CREATE TABLE events (
    id         INT          NOT NULL AUTO_INCREMENT PRIMARY KEY,
    device_id  VARCHAR(255) NOT NULL,
    level      ENUM('low', 'medium', 'high') NOT NULL DEFAULT 'low',
    message    TEXT         NOT NULL,
    created_at TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT fk_event_device
        FOREIGN KEY (device_id) REFERENCES devices(device_id)
        ON DELETE CASCADE
);
