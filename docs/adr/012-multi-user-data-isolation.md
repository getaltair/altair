# ADR-012: Multi-User Data Isolation

| Field             | Value           |
| ----------------- | --------------- |
| **Status**        | Accepted        |
| **Date**          | 2026-01-14      |
| **Deciders**      | Robert Hamilton |
| **Supersedes**    | —               |
| **Superseded by** | —               |

## Context

Altair is designed as a self-hosted application. In household scenarios, multiple people may want
to use the same server instance without sharing data. This raises questions about:

1. **Data isolation**: How to prevent users from seeing each other's content
2. **Authentication**: How users prove their identity
3. **Authorization**: What permissions exist beyond "user can access their own data"
4. **Storage**: How to handle per-user storage quotas
5. **Admin capabilities**: What server administrators can and cannot do

We needed to balance security (strict isolation) with practicality (households aren't adversarial).

## Decision

We chose **complete data isolation at the repository layer** with **invite-only registration**
and **admin accounts that cannot view user content**.

### Architecture

```
User Entity
├── username, email (optional), password_hash (Argon2)
├── role: admin | member
├── status: active | disabled | deleted
├── storage_used, storage_quota
└── All other entities have user_id foreign key

Repository Layer
├── All queries filtered by authenticated user_id
├── No cross-user data access
└── Admin cannot impersonate or view user content

Authentication
├── JWT tokens with user scope
├── Invite-only registration (default)
├── Multiple devices per user
└── Remote device revocation
```

### Key Design Decisions

1. **Repository-level filtering**: All queries include `WHERE user_id = ?` automatically
2. **Admin cannot view content**: Admins manage accounts, not access data
3. **Invite-only by default**: Admin generates invite codes for new users
4. **Storage quotas**: Optional per-user limits with clear enforcement
5. **Disabled user export**: Users can export data even when disabled

## Alternatives Considered

### Separate Database per User

Each user gets their own database instance:

**Rejected because:**
- Resource overhead (connection pools per user)
- Complicates sync and backup
- Overkill for household trust model
- Makes future sharing features harder

### Shared Data with Access Control

Content is shared by default, users explicitly restrict:

**Rejected because:**
- Privacy leak risk (forgot to restrict)
- Cognitive burden (must think about sharing)
- Doesn't match "personal productivity" mental model
- Complex permission model

### Admin Can View All Data

Administrators have superuser access:

**Rejected because:**
- Privacy concern even in households
- Single point of data access control
- Not necessary for admin tasks
- Could enable abuse

## Consequences

### Positive

- **Strong isolation**: Users cannot accidentally see each other's data
- **Simple mental model**: "My data is mine, period"
- **Household-friendly**: Share a server without sharing data
- **Clear admin scope**: Admins manage accounts, not content
- **Future-ready**: Explicit sharing can be added without breaking isolation

### Negative

- **No collaboration in v1**: Users cannot share Initiatives or content
- **Admin limitations**: Cannot help users by viewing their content
- **Query overhead**: Every query includes user_id filter
- **Duplicate content**: Users cannot share a Note or Item

### Mitigations

- **Future sharing**: Architecture supports adding explicit sharing later
- **User export**: Users can export their own data for manual sharing
- **Admin diagnostics**: Admins see aggregate stats, not content

## Implementation Notes

### User Entity

```kotlin
data class User(
    val id: String,
    val username: String,
    val email: String?,
    val passwordHash: String,
    val role: UserRole,
    val status: UserStatus,
    val storageUsed: Long,
    val storageQuota: Long?,
    val createdAt: Instant,
    val lastLoginAt: Instant?
)

enum class UserRole { ADMIN, MEMBER }
enum class UserStatus { ACTIVE, DISABLED, DELETED }
```

### Repository Pattern

```kotlin
class QuestRepository(
    private val db: Database,
    private val auth: AuthContext
) {
    // All queries automatically scoped to current user
    suspend fun findAll(): List<Quest> =
        db.query(
            "SELECT * FROM quest WHERE user_id = \$userId AND deleted_at IS NULL",
            mapOf("userId" to auth.currentUserId)
        )
    
    suspend fun create(quest: QuestCreate): Quest {
        // user_id injected automatically
        val created = quest.copy(userId = auth.currentUserId)
        return db.create("quest", created)
    }
}
```

### Authentication Flow

```
Registration (Invite-Only):
1. Admin generates invite code
2. User registers with invite code
3. Server validates code, creates user account
4. User receives JWT token

Login:
1. User provides username/password
2. Server validates credentials (Argon2 verify)
3. Server issues JWT token with user scope
4. Token stored in secure platform storage
```

### Storage Quota Enforcement

```kotlin
suspend fun uploadAttachment(file: ByteArray): Result<Attachment> {
    val user = userRepository.getCurrent()
    val quota = user.storageQuota ?: Long.MAX_VALUE
    
    if (user.storageUsed + file.size > quota) {
        return Result.failure(StorageQuotaExceeded(
            required = file.size,
            available = quota - user.storageUsed
        ))
    }
    
    val attachment = storageService.upload(file)
    userRepository.incrementStorageUsed(file.size)
    return Result.success(attachment)
}
```

### Admin Capabilities

| Capability | Allowed | Why |
|------------|---------|-----|
| Create users | ✓ | Account management |
| Generate invite codes | ✓ | Registration control |
| Disable users | ✓ | Access control |
| Reset passwords | ✓ | Account recovery |
| Set storage quotas | ✓ | Resource management |
| View server stats | ✓ | Operational monitoring |
| View user content | ✗ | Privacy protection |
| Impersonate users | ✗ | Security boundary |
| Export user data | ✗ | User agency |

### Disabled User Export

Users who are disabled can still log in with a limited session:
- Can view their data (read-only)
- Can initiate data export
- Cannot create, update, or delete content
- Cannot sync from mobile devices

## References

- [altair-prd-core.md §14](../requirements/altair-prd-core.md) — Authentication & Multi-User
- [persistence.md](../architecture/persistence.md) — Multi-user schema design
- [ADR-007](./007-docker-compose-deployment.md) — Server deployment

---

_Decision made: 2026-01-14_
