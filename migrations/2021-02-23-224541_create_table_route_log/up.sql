CREATE TABLE route_log (
    id              INTEGER         PRIMARY KEY     AUTO_INCREMENT  NOT NULL,
    time            BIGINT                                          NOT NULL,
    username        VARCHAR(128)                                    NOT NULL,
    route           TEXT                                            NOT NULL,
    data            TEXT
)