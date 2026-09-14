-- Stage 1.1 identity schema
-- SQL patterns: DROP, UUID DEFAULT, FK CASCADE, composite PK, INSERT…SELECT seed
-- Stored procedures: not used here (seed is plain SQL)

DROP TABLE IF EXISTS schema_migrations_smoke;

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL
);

CREATE TABLE permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code TEXT NOT NULL UNIQUE
);

CREATE TABLE role_permissions (
    role_id UUID NOT NULL,
    permission_id UUID NOT NULL,
    PRIMARY KEY (role_id, permission_id),
    CONSTRAINT fk_role_permissions_role
        FOREIGN KEY (role_id) REFERENCES roles (id) ON DELETE CASCADE,
    CONSTRAINT fk_role_permissions_permission
        FOREIGN KEY (permission_id) REFERENCES permissions (id) ON DELETE CASCADE
);

CREATE TABLE user_roles (
    user_id UUID NOT NULL,
    role_id UUID NOT NULL,
    PRIMARY KEY (user_id, role_id),
    CONSTRAINT fk_user_roles_user
        FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    CONSTRAINT fk_user_roles_role
        FOREIGN KEY (role_id) REFERENCES roles (id) ON DELETE CASCADE
);

CREATE TABLE refresh_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    token_hash TEXT NOT NULL,
    family_id UUID NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    revoked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_refresh_tokens_user
        FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

INSERT INTO roles (code, name) VALUES
    ('customer', 'Customer'),
    ('ops_maker', 'Ops Maker'),
    ('ops_checker', 'Ops Checker');

INSERT INTO permissions (code) VALUES
    ('identity:self'),
    ('accounts:read'),
    ('accounts:open'),
    ('transfers:create'),
    ('ops:transfers:read'),
    ('ops:actions:propose'),
    ('ops:actions:approve');

INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r
CROSS JOIN permissions p
WHERE r.code = 'customer'
  AND p.code IN ('identity:self', 'accounts:read', 'accounts:open', 'transfers:create');

INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r
CROSS JOIN permissions p
WHERE r.code = 'ops_maker'
  AND p.code IN ('ops:transfers:read', 'ops:actions:propose');

INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r
CROSS JOIN permissions p
WHERE r.code = 'ops_checker'
  AND p.code IN ('ops:transfers:read', 'ops:actions:approve');


INSERT INTO roles (id, code, name)
VALUES
    ('00000000-0000-0000-0000-000000000001', 'customer', 'Customer'),
    ('00000000-0000-0000-0000-000000000002', 'ops_maker', 'Operations Maker'),
    ('00000000-0000-0000-0000-000000000003', 'ops_checker', 'Operations Checker');

INSERT INTO permissions (id, code)
VALUES
    ('10000000-0000-0000-0000-000000000001', 'identity:self'),
    ('10000000-0000-0000-0000-000000000002', 'customers:read'),
    ('10000000-0000-0000-0000-000000000003', 'customers:write'),
    ('10000000-0000-0000-0000-000000000004', 'accounts:read'),
    ('10000000-0000-0000-0000-000000000005', 'accounts:open'),
    ('10000000-0000-0000-0000-000000000006', 'transfers:create'),
    ('10000000-0000-0000-0000-000000000007', 'transfers:read'),
    ('10000000-0000-0000-0000-000000000008', 'ledger:read'),
    ('10000000-0000-0000-0000-000000000009', 'ops:transfers:read'),
    ('10000000-0000-0000-0000-000000000010', 'ops:actions:propose'),
    ('10000000-0000-0000-0000-000000000011', 'ops:actions:approve'),
    ('10000000-0000-0000-0000-000000000012', 'ops:recon:read'),
    ('10000000-0000-0000-0000-000000000013', 'ops:recon:act'),
    ('10000000-0000-0000-0000-000000000014', 'ops:fraud:read'),
    ('10000000-0000-0000-0000-000000000015', 'ops:fraud:act');


INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r
         CROSS JOIN permissions p
WHERE
    (r.code = 'customer' AND p.code IN (
                                        'identity:self',
                                        'accounts:read',
                                        'accounts:open',
                                        'transfers:create',
                                        'transfers:read',
                                        'ledger:read'
        ))
   OR
    (r.code = 'ops_maker' AND p.code IN (
                                         'customers:read',
                                         'customers:write',
                                         'accounts:read',
                                         'ops:transfers:read',
                                         'ops:actions:propose',
                                         'ops:recon:read',
                                         'ops:recon:act',
                                         'ops:fraud:read',
                                         'ops:fraud:act'
        ))
   OR
    (r.code = 'ops_checker' AND p.code IN (
                                           'customers:read',
                                           'accounts:read',
                                           'ops:transfers:read',
                                           'ops:actions:approve',
                                           'ops:recon:read',
                                           'ops:fraud:read'
        ));

CREATE INDEX idx_refresh_tokens_user_id
    ON refresh_tokens (user_id);

CREATE INDEX idx_refresh_tokens_family_id
    ON refresh_tokens (family_id);