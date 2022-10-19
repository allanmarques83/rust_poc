-- Add migration script here
INSERT INTO users (
    uuid, username, email, password_hash, status
) VALUES (
    '3e3dd4ae-3c37-40c6-aa64-7061f284ce28', 'admin', 'admin@example.com', '123456', 1
);