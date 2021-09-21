CREATE TABLE submissions (
    id          INTEGER         NOT NULL    PRIMARY KEY AUTO_INCREMENT,
    user        VARCHAR(128)    NOT NULL,
    course      VARCHAR(128)    NOT NULL,
    taskid      INTEGER         NOT NULL,
    timestamp   BIGINT          NOT NULL,
    content     TEXT            NOT NULL,
    resultType  VARCHAR(128)    NOT NULL,
    simplified  TEXT            NOT NULL,
    details     TEXT            NOT NULL,
    score       FLOAT           NOT NULL
)