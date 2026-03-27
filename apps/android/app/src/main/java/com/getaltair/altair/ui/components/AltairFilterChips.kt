package com.getaltair.altair.ui.components

import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.material3.FilterChip
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp

data class FilterChipOption<T>(val label: String, val value: T)

@Composable
fun <T> AltairFilterChips(
    options: List<FilterChipOption<T>>,
    selectedValue: T,
    onSelect: (T) -> Unit,
    modifier: Modifier = Modifier,
) {
    Row(
        modifier = modifier
            .horizontalScroll(rememberScrollState())
            .padding(horizontal = 16.dp, vertical = 4.dp),
    ) {
        options.forEach { option ->
            FilterChip(
                selected = selectedValue == option.value,
                onClick = { onSelect(option.value) },
                label = { Text(option.label) },
                modifier = Modifier.padding(end = 8.dp),
            )
        }
    }
}
