CREATE TABLE program_capabilities (
    program_id     INT  NOT NULL,
    capability_id  INT  NOT NULL,

    PRIMARY KEY (program_id, capability_id),

    CONSTRAINT fk_program
        FOREIGN KEY (program_id)
        REFERENCES programs(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,

    CONSTRAINT fk_capability
        FOREIGN KEY (capability_id)
        REFERENCES capabilities(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
