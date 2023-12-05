CREATE TABLE tickets IF NOT EXISTS (
       id UUID DEFAULT Uuid_generate_v4 () PRIMARY KEY,
       status_id UUID NOT NULL REFERENCES statuses(id),
       submitted_by UUID NOT NULL REFERENCES users(id),
       date_submitted timestamptz NOT NULL,
       ticket_type_id UUID REFERENCES ticket_types(id)
)
