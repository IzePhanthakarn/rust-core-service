-- This file should undo anything in `up.sql`

DROP TABLE IF EXISTS task_comments;
DROP TABLE IF EXISTS tasks;
DROP TABLE IF EXISTS sprints;
DROP TABLE IF EXISTS board_columns;
DROP TABLE IF EXISTS boards;
DROP TABLE IF EXISTS project_notes;
DROP TABLE IF EXISTS project_members;
DROP TABLE IF EXISTS projects;

DROP TYPE IF EXISTS task_priority;
DROP TYPE IF EXISTS task_type;
DROP TYPE IF EXISTS note_type;
DROP TYPE IF EXISTS project_status;
