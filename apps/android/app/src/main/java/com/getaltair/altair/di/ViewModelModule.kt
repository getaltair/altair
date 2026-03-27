package com.getaltair.altair.di

import com.getaltair.altair.ui.guidance.checkin.CheckinViewModel
import com.getaltair.altair.ui.guidance.initiative.InitiativeDetailViewModel
import com.getaltair.altair.ui.guidance.initiative.InitiativeListViewModel
import com.getaltair.altair.ui.guidance.quest.QuestDetailViewModel
import com.getaltair.altair.ui.guidance.routine.RoutineDetailViewModel
import com.getaltair.altair.ui.guidance.routine.RoutineListViewModel
import com.getaltair.altair.ui.guidance.today.TodayViewModel
import com.getaltair.altair.ui.settings.SettingsViewModel
import org.koin.core.module.dsl.viewModelOf
import org.koin.dsl.module

val viewModelModule = module {
    viewModelOf(::TodayViewModel)
    viewModelOf(::InitiativeListViewModel)
    viewModelOf(::InitiativeDetailViewModel)
    viewModelOf(::QuestDetailViewModel)
    viewModelOf(::RoutineListViewModel)
    viewModelOf(::RoutineDetailViewModel)
    viewModelOf(::CheckinViewModel)
    viewModelOf(::SettingsViewModel)
}
