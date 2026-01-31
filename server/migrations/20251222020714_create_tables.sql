CREATE TYPE conversation_kind AS ENUM ('direct', 'group');
CREATE TYPE message_kind AS ENUM ('text', 'image');

CREATE TABLE users (
    id TEXT NOT NULL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE users IS 'User accounts and profile information';
COMMENT ON COLUMN users.id IS 'Clerk user ID';


CREATE TABLE conversations (
    id TEXT NOT NULL PRIMARY KEY,
    kind conversation_kind NOT NULL,
    title TEXT,
    last_message_id TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT direct_no_title CHECK (kind != 'direct' OR title IS NULL),
    CONSTRAINT group_has_title CHECK (kind != 'group' OR title IS NOT NULL)
);

CREATE INDEX idx_conversations_updated_at ON conversations (updated_at DESC);
CREATE INDEX idx_conversations_kind_updated ON conversations (kind, updated_at DESC);

COMMENT ON TABLE conversations IS 'Chat conversations between users';
COMMENT ON COLUMN conversations.last_message_id IS 'Most recent message ID, NULL if conversation has no messages yet';


CREATE TABLE messages (
    id TEXT NOT NULL PRIMARY KEY,
    conversation_id TEXT NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
    sender_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    kind message_kind NOT NULL,
    edited BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ
);

CREATE INDEX idx_messages_conversation_id ON messages (conversation_id, created_at DESC);
CREATE INDEX idx_messages_sender_id ON messages (sender_id);
CREATE INDEX idx_messages_created_at ON messages (created_at DESC);

COMMENT ON TABLE messages IS 'Messages sent in conversations';
COMMENT ON COLUMN messages.content IS 'Message text content or image URL depending on kind';
COMMENT ON COLUMN messages.edited IS 'True if message has been edited';
COMMENT ON COLUMN messages.updated_at IS 'Timestamp of last edit, NULL if never edited';


ALTER TABLE conversations
  ADD CONSTRAINT fk_last_message
  FOREIGN KEY (last_message_id)
  REFERENCES messages(id)
  ON DELETE SET NULL;

CREATE TABLE user_conversations (
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    conversation_id TEXT NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
    last_read_message_id TEXT REFERENCES messages(id) ON DELETE SET NULL,
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_seen_at TIMESTAMPTZ,

    PRIMARY KEY (user_id, conversation_id)
);

CREATE INDEX idx_user_conversations_conversation_id ON user_conversations (conversation_id);

COMMENT ON TABLE user_conversations IS 'Junction table tracking user participation and read status in conversations';
COMMENT ON COLUMN user_conversations.last_read_message_id IS 'Last message this user has read in the conversation';
COMMENT ON COLUMN user_conversations.last_seen_at IS 'Last time user was active in this conversation (for presence)';
