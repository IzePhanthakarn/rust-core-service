CREATE TABLE work_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(100) NOT NULL,
    content TEXT NOT NULL,
    mood_score INTEGER NOT NULL,
    productivity_score INTEGER NOT NULL,
    is_draft BOOLEAN NOT NULL DEFAULT false,
    date_logged TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_work_logs_date_logged ON work_logs(date_logged);

CREATE TABLE work_log_tags (
    log_id UUID NOT NULL REFERENCES work_logs(id) ON DELETE CASCADE,
    work_tag varchar(50) NOT NULL,
    PRIMARY KEY (log_id, work_tag)
);

CREATE INDEX idx_work_log_tags_tag ON work_log_tags(work_tag);