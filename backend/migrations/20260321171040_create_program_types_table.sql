CREATE TABLE program_types (
    id          INT             NOT NULL AUTO_INCREMENT PRIMARY KEY,
    name        VARCHAR(255)    NOT NULL UNIQUE,
    color       CHAR(7)         NOT NULL -- e.g. "#FF5733"
    created_at  TIMESTAMP       NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP       NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
);

INSERT INTO program_types (name, color) VALUES
('Spyware',         '#8c564b'),
('Keylogger',       '#ff7f0e'),
('Cookie Stealer',  '#2ca02c'),
('RAT',             '#d62728'),
('Ransomware',      '#9467bd'),
('Loader',          '#9b0a7b')
('Multipurpose',    '#1f77b4');