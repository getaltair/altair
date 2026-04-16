package com.getaltair.altair.data.local.entity

import androidx.room.ColumnInfo
import androidx.room.Entity
import androidx.room.PrimaryKey

@Entity(tableName = "users")
data class UserEntity(
    @PrimaryKey val id: String,
    @ColumnInfo(name = "password_hash") val passwordHash: String?,
    @ColumnInfo(name = "email") val email: String,
    @ColumnInfo(name = "display_name") val displayName: String?,
    @ColumnInfo(name = "is_admin") val isAdmin: Int,
    @ColumnInfo(name = "status") val status: String,
    @ColumnInfo(name = "created_at") val createdAt: String,
    @ColumnInfo(name = "updated_at") val updatedAt: String,
    @ColumnInfo(name = "deleted_at") val deletedAt: String?,
)
