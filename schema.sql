CREATE TABLE team_members (
    id bigint NOT NULL,
    role character varying NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    company_id bigint NOT NULL,
    user_id bigint NOT NULL,
    CONSTRAINT team_members_company_id_cae6a900_fk_users_company_id FOREIGN KEY (company_id) REFERENCES companies(id),
    CONSTRAINT team_members_user_id_cfdfac1d_fk_users_id FOREIGN KEY (user_id) REFERENCES users(id),
    PRIMARY KEY (id)
);

