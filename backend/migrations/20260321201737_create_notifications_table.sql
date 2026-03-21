CREATE TABLE notifications (
    id              INT                                                                                     NOT NULL AUTO_INCREMENT PRIMARY KEY,

    -- target
    admin_id        INT                                                                                     NULL,     -- NULL = broadcast to all admins
    
    -- source (what triggered it, all nullable since not every event has all sources)
    device_id       INT                                                                                     NULL,
    program_id      INT                                                                                     NULL,
    upload_id       INT                                                                                     NULL,
    ticket_id       INT                                                                                     NULL,

    -- content
    type            ENUM('device_connected', 'device_disconnected', 'device_died',
                         'file_uploaded', 'ticket_exhausted', 'ticket_expired',
                         'upload_rejected', 'new_program', 'suspicious_activity')       NOT NULL,
    severity        ENUM('info', 'warning', 'critical')                                 NOT NULL DEFAULT 'info',
    title           VARCHAR(255)                                                        NOT NULL,
    body            TEXT                                                                NULL,

    -- state
    is_read         BOOLEAN                                                             NOT NULL DEFAULT FALSE,
    created_at      TIMESTAMP                                                           NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT fk_notification_admin
        FOREIGN KEY (admin_id)
        REFERENCES admins(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,

    CONSTRAINT fk_notification_device
        FOREIGN KEY (device_id)
        REFERENCES devices(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,

    CONSTRAINT fk_notification_program
        FOREIGN KEY (program_id)
        REFERENCES programs(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,

    CONSTRAINT fk_notification_upload
        FOREIGN KEY (upload_id)
        REFERENCES uploads(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,

    CONSTRAINT fk_notification_ticket
        FOREIGN KEY (ticket_id)
        REFERENCES upload_tickets(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);