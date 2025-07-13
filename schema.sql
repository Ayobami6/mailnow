CREATE TABLE users (
    id bigint NOT NULL,
    password character varying NOT NULL,
    last_login timestamp with time zone NULL,
    is_superuser boolean NOT NULL,
    email character varying NOT NULL,
    firstname character varying NULL,
    lastname character varying NULL,
    is_active boolean NOT NULL,
    is_staff boolean NOT NULL,
    mfa_enabled boolean NOT NULL,
    email_verifield boolean NOT NULL,
    date_joined timestamp with time zone NOT NULL,
    PRIMARY KEY (id)
);

