CREATE TABLE api_keys (
    id bigint NOT NULL,
    name character varying NOT NULL,
    api_key character varying NOT NULL,
    created_at timestamp with time zone NOT NULL,
    last_used timestamp with time zone NULL,
    expires_at timestamp with time zone NULL,
    is_active boolean NOT NULL,
    company_id bigint NOT NULL,
    permission character varying NULL,
    CONSTRAINT api_keys_company_id_8e87fa92_fk_users_company_id FOREIGN KEY (company_id) REFERENCES companies(id),
    PRIMARY KEY (id)
);

