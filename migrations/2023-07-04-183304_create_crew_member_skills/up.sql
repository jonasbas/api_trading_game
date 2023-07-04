-- Your SQL goes here
CREATE TYPE CrewSkill AS ENUM (
    'steering', 'trading', 'rowing'
);

CREATE TABLE crew_member_skills (
    crew_id INTEGER REFERENCES crew_members(id) ON DELETE CASCADE,
    skill CrewSkill NOT NULL,
    value INTEGER NOT NULL,
    PRIMARY KEY(crew_id, skill)
);
