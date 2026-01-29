package com.getaltair.server

import com.getaltair.server.routes.*
import com.getaltair.altair.shared.repository.*
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.plugins.statuspages.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import org.koin.ktor.ext.inject

/**
 * Configure application routing with all API endpoints.
 *
 * All routes except health check require JWT authentication.
 * Routes are organized by domain module:
 * - /api/auth - Authentication (public + protected)
 * - /api/guidance - Quests, Epics, Checkpoints, Energy
 * - /api/knowledge - Notes, Folders, Tags
 * - /api/tracking - Items, Locations, Containers, Templates
 * - /api/inbox - Universal inbox capture and triage
 * - /api/initiatives - Cross-module organization
 * - /api/routines - Recurring tasks
 * - /api/sync - Multi-device sync (Phase 12)
 * - /api/ai - AI services (Phase 13)
 */
fun Application.configureRouting() {
    // Inject repositories from Koin
    val questRepository by inject<QuestRepository>()
    val epicRepository by inject<EpicRepository>()
    val noteRepository by inject<NoteRepository>()
    val itemRepository by inject<ItemRepository>()
    val inboxRepository by inject<InboxRepository>()
    val initiativeRepository by inject<InitiativeRepository>()
    val routineRepository by inject<RoutineRepository>()

    install(StatusPages) {
        exception<Throwable> { call, cause ->
            call.respondText(text = "500: $cause", status = HttpStatusCode.InternalServerError)
        }
    }
    routing {
        // Health check endpoint (no auth required)
        get("/") {
            call.respondText("Altair API Server")
        }

        get("/health") {
            call.respondText("OK")
        }

        // Wire all route modules
        authRoutes()
        guidanceRoutes(questRepository, epicRepository)
        knowledgeRoutes(noteRepository)
        trackingRoutes(itemRepository)
        inboxRoutes(inboxRepository)
        initiativeRoutes(initiativeRepository)
        routineRoutes(routineRepository)
        syncRoutes()
        aiRoutes()
    }
}
