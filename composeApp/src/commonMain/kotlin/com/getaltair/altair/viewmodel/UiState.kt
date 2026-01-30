package com.getaltair.altair.viewmodel

/**
 * Generic sealed interface for representing UI state.
 *
 * Provides a type-safe way to handle loading, success, and error states
 * in ViewModels and UI components.
 *
 * @param T The type of data contained in the success state
 */
sealed interface UiState<out T> {
    /**
     * Initial loading state, displayed while data is being fetched.
     */
    data object Loading : UiState<Nothing>

    /**
     * Success state containing the loaded data.
     *
     * @param data The successfully loaded data
     */
    data class Success<T>(val data: T) : UiState<T>

    /**
     * Error state containing an error message.
     *
     * @param message Human-readable error message to display
     */
    data class Error(val message: String) : UiState<Nothing>
}
