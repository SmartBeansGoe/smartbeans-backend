CREATE TABLE error_reports (
    id              INTEGER         PRIMARY KEY     AUTO_INCREMENT  NOT NULL,
    time            BIGINT                                          NOT NULL,
    username        VARCHAR(128)                                    NOT NULL,
    message         TEXT                                            NOT NULL
)