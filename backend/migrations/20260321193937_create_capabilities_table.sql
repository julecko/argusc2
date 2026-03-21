CREATE TABLE capabilities (
    id          INT           NOT NULL AUTO_INCREMENT PRIMARY KEY,
    name        VARCHAR(100)  NOT NULL UNIQUE,  -- e.g., "Keylogger", "RAT", "Stealer"
    description TEXT          NULL,
    created_at  TIMESTAMP     NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO capabilities (name, description) VALUES
('Keylogger', 'Records keystrokes on target machine'),
('RAT', 'Remote Access Trojan functionality'),
('Cookie Stealer', 'Extracts browser cookies from target'),
('Ransomware', 'Encrypts files and demands ransom'),
('Spyware', 'Monitors user activity stealthily');
