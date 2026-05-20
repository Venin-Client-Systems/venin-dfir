CREATE TABLE urls (
    id INTEGER PRIMARY KEY,
    url LONGVARCHAR,
    title LONGVARCHAR,
    visit_count INTEGER DEFAULT 0 NOT NULL,
    typed_count INTEGER DEFAULT 0 NOT NULL,
    last_visit_time INTEGER NOT NULL,
    hidden INTEGER DEFAULT 0 NOT NULL
);

CREATE TABLE visits (
    id INTEGER PRIMARY KEY,
    url INTEGER NOT NULL,
    visit_time INTEGER NOT NULL,
    from_visit INTEGER,
    transition INTEGER DEFAULT 0 NOT NULL,
    segment_id INTEGER
);

INSERT INTO urls (id, url, title, visit_count, typed_count, last_visit_time, hidden)
VALUES
    (1, 'https://example.org/research', 'Research Notes', 3, 1, 13348638245000000, 0),
    (2, 'https://example.com/login', 'Example Login', 1, 0, 13348584000000000, 0);

INSERT INTO visits (id, url, visit_time, from_visit, transition, segment_id)
VALUES
    (1, 2, 13348584000000000, 0, 805306368, 0),
    (2, 1, 13348638245000000, 1, 268435456, 0);
