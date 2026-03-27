package com.getaltair.altair.di

import android.content.Context
import androidx.room.Room
import com.getaltair.altair.data.local.database.AltairDatabase
import org.koin.android.ext.koin.androidContext
import org.koin.dsl.module

val databaseModule = module {

    single<AltairDatabase> { provideDatabase(androidContext()) }

    single { get<AltairDatabase>().userDao() }
    single { get<AltairDatabase>().householdDao() }
    single { get<AltairDatabase>().initiativeDao() }
    single { get<AltairDatabase>().epicDao() }
    single { get<AltairDatabase>().questDao() }
    single { get<AltairDatabase>().routineDao() }
    single { get<AltairDatabase>().tagDao() }
    single { get<AltairDatabase>().checkinDao() }
    single { get<AltairDatabase>().focusSessionDao() }
    single { get<AltairDatabase>().entityRelationDao() }
    single { get<AltairDatabase>().knowledgeNoteDao() }
    single { get<AltairDatabase>().knowledgeNoteSnapshotDao() }
    single { get<AltairDatabase>().trackingItemDao() }
    single { get<AltairDatabase>().trackingItemEventDao() }
    single { get<AltairDatabase>().trackingLocationDao() }
    single { get<AltairDatabase>().trackingCategoryDao() }
    single { get<AltairDatabase>().trackingShoppingListDao() }
    single { get<AltairDatabase>().trackingShoppingListItemDao() }
    single { get<AltairDatabase>().attachmentMetadataDao() }
}

@Suppress("DEPRECATION")
private fun provideDatabase(context: Context): AltairDatabase =
    Room.databaseBuilder(
        context.applicationContext,
        AltairDatabase::class.java,
        "altair.db",
    ).fallbackToDestructiveMigration().build()
