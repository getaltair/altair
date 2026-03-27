package com.getaltair.altair.ui.capture

import android.content.Intent
import android.os.Bundle
import androidx.activity.ComponentActivity
import com.getaltair.altair.domain.entity.ContentType
import com.getaltair.altair.domain.entity.KnowledgeNote
import com.getaltair.altair.domain.repository.KnowledgeNoteRepository
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import org.koin.android.ext.android.inject
import java.time.Instant
import java.util.UUID

class ShareReceiver : ComponentActivity() {
    private val knowledgeNoteRepository: KnowledgeNoteRepository by inject()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        handleShareIntent(intent)
        finish()
    }

    private fun handleShareIntent(intent: Intent) {
        if (intent.action != Intent.ACTION_SEND) return
        val sharedText = intent.getStringExtra(Intent.EXTRA_TEXT) ?: return
        val now = Instant.now()
        val note = KnowledgeNote(
            id = UUID.randomUUID(),
            userId = UUID.randomUUID(), // TODO: use real userId from session
            householdId = null,
            initiativeId = null,
            title = "Shared Note",
            content = sharedText,
            contentType = ContentType.PLAIN,
            isPinned = false,
            createdAt = now,
            updatedAt = now,
        )
        CoroutineScope(Dispatchers.IO).launch {
            knowledgeNoteRepository.create(note)
        }
    }
}
