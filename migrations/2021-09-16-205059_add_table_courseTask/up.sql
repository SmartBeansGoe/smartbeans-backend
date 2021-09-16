CREATE TABLE courseTask (
    course          VARCHAR(128)    NOT NULL,
    taskid          INTEGER         NOT NULL,
    tags            TEXT            NOT NULL    DEFAULT '[]',
    orderBy         INTEGER         NOT NULL    DEFAULT 0,
    prerequisites   TEXT            NOT NULL    DEFAULT '[]',
    PRIMARY KEY (course, taskid)
)