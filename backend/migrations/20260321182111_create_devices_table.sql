CREATE TABLE devices (
    id              INT                                                                     NOT NULL AUTO_INCREMENT PRIMARY KEY,

    -- identity
    device_id       VARCHAR(255)                                                            NOT NULL, -- id sended each time by device to track devices
    program_id      INT                                                                     NOT NULL, -- which implant is running
    hostname        VARCHAR(255)                                                            NULL,
    username        VARCHAR(255)                                                            NULL, -- logged-in user on the victim machine

    -- network
    ip_internal     VARCHAR(45)                                                             NULL,     -- LAN IP (IPv4 or IPv6)
    ip_external     VARCHAR(45)                                                             NULL,     -- WAN IP as seen by server
    mac_address     CHAR(17)                                                                NULL,     -- e.g. 'AA:BB:CC:DD:EE:FF'

    -- OS / hardware
    os              ENUM('windows', 'linux', 'android', 'mac')                              NULL,
    os_version      VARCHAR(100)                                                            NULL,     -- e.g. 'Windows 11 23H2'
    arch            ENUM('x86', 'x64', 'arm', 'arm64')                                      NULL,
    cpu             VARCHAR(255)                                                            NULL,     -- e.g. 'Intel Core i7-12700K'
    cpu_cores       TINYINT UNSIGNED                                                        NULL,
    ram_bytes       BIGINT UNSIGNED                                                         NULL,     -- total RAM
    disk_bytes      BIGINT UNSIGNED                                                         NULL,     -- total primary disk size

    is_elevated     BOOLEAN                                                                 NOT NULL DEFAULT FALSE, -- running as SYSTEM / root

    country_code    CHAR(2)                                                                 NULL,     -- ISO 3166-1 alpha-2
    city            VARCHAR(100)                                                            NULL,
    timezone        VARCHAR(64)                                                             NULL,     -- e.g. 'Europe/Bratislava'

    first_seen      TIMESTAMP                                                               NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_seen       TIMESTAMP                                                               NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    CONSTRAINT fk_device_program
        FOREIGN KEY (program_id)
        REFERENCES programs(id)
        ON DELETE RESTRICT
        ON UPDATE CASCADE
);
