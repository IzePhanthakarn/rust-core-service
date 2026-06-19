-- Your SQL goes here

-- Main category / "folder"
CREATE TABLE todo_lists (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(100) NOT NULL,
    description VARCHAR(3000),
    color VARCHAR(20),
    position INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_todo_lists_user_id ON todo_lists(user_id);

-- Individual tasks, each belongs to one todo_list (one-to-many)
CREATE TABLE todo_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    list_id UUID NOT NULL REFERENCES todo_lists(id) ON DELETE CASCADE,
    title VARCHAR(100) NOT NULL,
    description VARCHAR(3000),
    is_completed BOOLEAN NOT NULL DEFAULT FALSE,
    due_date TIMESTAMP WITH TIME ZONE,
    position INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_todo_items_list_id ON todo_items(list_id);
