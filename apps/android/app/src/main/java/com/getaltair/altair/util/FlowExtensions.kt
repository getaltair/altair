package com.getaltair.altair.util

import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.flow.map
import timber.log.Timber

fun <T> Flow<T>.catchAndLog(
    tag: String = "Flow",
    onError: ((Throwable) -> Unit)? = null,
): Flow<T> = catch { e ->
    Timber.e(e, "$tag: ${e.message}")
    onError?.invoke(e)
}

fun <E, D> Flow<List<E>>.mapToDomain(transform: (E) -> D): Flow<List<D>> =
    map { list -> list.map(transform) }

suspend fun <T> MutableStateFlow<T>.emitCatching(
    value: T,
    onError: ((Throwable) -> Unit)? = null,
) {
    try {
        emit(value)
    } catch (e: Exception) {
        Timber.e(e, "Failed to emit state: ${e.message}")
        onError?.invoke(e)
    }
}
