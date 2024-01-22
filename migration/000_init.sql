-- create a table
CREATE TABLE mock_table (
    id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name TEXT NOT NULL
);
-- add test data
INSERT INTO mock_table (name)
VALUES ('test row 1'),
    ('test row 2');