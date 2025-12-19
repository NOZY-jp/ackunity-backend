-- 拡張機能有効化
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- ユーザー
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    userid TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    displayname TEXT,
    password_hash TEXT NOT NULL,
    tfa_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    tfa_secret TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 組織
CREATE TABLE organizations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    orgid TEXT NOT NULL UNIQUE,
    orgname TEXT NOT NULL,
    join_code TEXT NOT NULL UNIQUE,
    join_password_hash TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 組織メンバーシップ
CREATE TABLE membership (
    org_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    permission TEXT NOT NULL,
    join_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (org_id, user_id)
);

-- 通知
CREATE TABLE notifications (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    org_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    source_type TEXT NOT NULL,
    importance INTEGER NOT NULL,
    title TEXT NOT NULL,
    content TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    starts_at TIMESTAMPTZ,
    ends_at TIMESTAMPTZ
);

-- リアクション種類
CREATE TABLE reaction_variants (
    id SERIAL PRIMARY KEY,
    emoji_code TEXT NOT NULL UNIQUE,
    label TEXT
);

-- インタラクション (既読/反応)
CREATE TABLE interactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    notification_id UUID NOT NULL REFERENCES notifications(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    reaction_id INTEGER REFERENCES reaction_variants(id),
    content TEXT,
    interacted_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 検索・未読カウント用インデックス
CREATE INDEX idx_interactions_notif_user ON interactions(notification_id, user_id);
CREATE INDEX idx_notifications_org_created ON notifications(org_id, created_at DESC);

-- 初期マスタデータ
INSERT INTO reaction_variants (emoji_code, label) VALUES 
('check', '確認済み'),
('eyes', '確認中'),
('rocket', '着手');
