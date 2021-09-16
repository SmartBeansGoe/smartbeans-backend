CREATE TABLE tasks (
    taskid          INTEGER NOT NULL    PRIMARY KEY,
    taskDescription TEXT    NOT NULL,
    solution        TEXT    NOT NULL,
    lang            TEXT    NOT NULL,
    tests           TEXT    NOT NULL
)