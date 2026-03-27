package com.getaltair.altair.di

import com.getaltair.altair.data.repository.CheckinRepositoryImpl
import com.getaltair.altair.data.repository.EpicRepositoryImpl
import com.getaltair.altair.data.repository.InitiativeRepositoryImpl
import com.getaltair.altair.data.repository.QuestRepositoryImpl
import com.getaltair.altair.data.repository.RoutineRepositoryImpl
import com.getaltair.altair.domain.repository.CheckinRepository
import com.getaltair.altair.domain.repository.EpicRepository
import com.getaltair.altair.domain.repository.InitiativeRepository
import com.getaltair.altair.domain.repository.QuestRepository
import com.getaltair.altair.domain.repository.RoutineRepository
import java.util.UUID
import org.koin.dsl.module

val repositoryModule = module {

    // Placeholder userId provider -- will be replaced with real auth when server sync is wired
    single<() -> UUID> { { UUID.fromString("00000000-0000-0000-0000-000000000001") } }

    single<InitiativeRepository> {
        InitiativeRepositoryImpl(
            initiativeDao = get(),
            userId = get(),
        )
    }

    single<EpicRepository> {
        EpicRepositoryImpl(epicDao = get())
    }

    single<QuestRepository> {
        QuestRepositoryImpl(
            questDao = get(),
            userId = get(),
        )
    }

    single<RoutineRepository> {
        RoutineRepositoryImpl(
            routineDao = get(),
            userId = get(),
        )
    }

    single<CheckinRepository> {
        CheckinRepositoryImpl(checkinDao = get())
    }
}
