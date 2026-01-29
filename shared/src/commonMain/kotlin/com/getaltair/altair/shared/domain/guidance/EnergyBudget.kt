package com.getaltair.altair.shared.domain.guidance

import com.getaltair.altair.shared.domain.common.Ulid
import kotlinx.datetime.LocalDate
import kotlinx.serialization.Serializable

/**
 * EnergyBudget: Daily capacity tracking for ADHD-aware task planning.
 *
 * Energy budgets represent the total cognitive/emotional capacity available for a given
 * day. This enables neurodivergent-friendly planning by constraining quest selection
 * based on realistic daily capacity rather than rigid time-based estimates.
 *
 * ## Budget Scale
 * Budget represents "energy units" available per day:
 * - **1-3**: Low energy days (minimal capacity, prioritize critical tasks only)
 * - **4-6**: Standard days (typical workload capacity)
 * - **7-10**: High energy days (can tackle demanding work)
 *
 * Default is 5, representing a typical day. Users adjust their daily budget based on
 * factors like sleep, stress, health, and external demands.
 *
 * ## Spent Calculation
 * The [spent] field is derived from the sum of energy costs of all quests marked
 * COMPLETED on this date. It is not user-editable directly; quest completion drives
 * this value.
 *
 * ## Over-Budget Warning
 * When [spent] exceeds [budget], the system can provide warnings to help users recognize
 * they're exceeding planned capacity, enabling earlier course correction.
 *
 * ## Daily Reset
 * Each day gets its own EnergyBudget record. This allows historical tracking of capacity
 * patterns over time, which can inform future planning and self-awareness.
 *
 * @property userId Owner of this energy budget
 * @property date The specific day this budget applies to
 * @property budget Total energy units available (1-10, default 5)
 * @property spent Energy units consumed by completed quests (auto-calculated)
 *
 * @throws IllegalArgumentException if budget is outside 1-10 range or spent is negative
 */
@Serializable
data class EnergyBudget(
    val userId: Ulid,
    val date: LocalDate,
    val budget: Int,
    val spent: Int
) {
    init {
        require(budget in 1..10) { "Energy budget must be in range 1-10, got $budget" }
        require(spent >= 0) { "Spent energy cannot be negative, got $spent" }
    }

    /**
     * Remaining energy units available for the day.
     *
     * Can be negative if over-budget (spent > budget).
     */
    val remaining: Int get() = budget - spent

    /**
     * Returns true if spent energy exceeds the planned budget.
     *
     * Useful for triggering warnings or UI indicators to help users recognize
     * they're over capacity.
     */
    val isOverBudget: Boolean get() = spent > budget

    /**
     * Percentage of budget consumed (0.0 to infinity).
     *
     * Values > 1.0 indicate over-budget condition.
     * Example: 0.6 = 60% used, 1.2 = 120% used (over-budget)
     */
    val percentUsed: Float get() = spent.toFloat() / budget
}
