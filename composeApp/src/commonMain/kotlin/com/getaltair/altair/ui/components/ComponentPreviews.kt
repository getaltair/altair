package com.getaltair.altair.ui.components

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.getaltair.altair.ui.theme.AltairTheme
import com.getaltair.altair.ui.theme.AltairThemeProvider

/**
 * Preview showcase for all Altair design system components.
 *
 * Use this composable to visually test and verify component styling.
 * Can be rendered in a test activity or preview window.
 */
@Suppress("ktlint:compose:modifier-missing-check")
@Composable
fun DesignSystemShowcase() {
    AltairThemeProvider {
        AltairSurface(modifier = Modifier.fillMaxSize()) {
            Column(
                modifier =
                    Modifier
                        .fillMaxSize()
                        .verticalScroll(rememberScrollState())
                        .padding(AltairTheme.Spacing.lg),
            ) {
                SectionHeader("Typography")
                TypographyShowcase()

                Spacer(modifier = Modifier.height(AltairTheme.Spacing.xl))
                SectionHeader("Buttons")
                ButtonShowcase()

                Spacer(modifier = Modifier.height(AltairTheme.Spacing.xl))
                SectionHeader("Text Fields")
                TextFieldShowcase()

                Spacer(modifier = Modifier.height(AltairTheme.Spacing.xl))
                SectionHeader("Cards")
                CardShowcase()

                Spacer(modifier = Modifier.height(AltairTheme.Spacing.xl))
                SectionHeader("Checkboxes")
                CheckboxShowcase()

                Spacer(modifier = Modifier.height(AltairTheme.Spacing.xl))
                SectionHeader("Dialogs")
                DialogShowcase()

                Spacer(modifier = Modifier.height(AltairTheme.Spacing.xl))
                SectionHeader("Dropdown Menus")
                DropdownShowcase()

                Spacer(modifier = Modifier.height(AltairTheme.Spacing.xl))
                SectionHeader("Progress Indicators")
                ProgressShowcase()
            }
        }
    }
}

@Composable
private fun SectionHeader(title: String) {
    AltairText(
        text = title,
        style = AltairTheme.Typography.headlineSmall,
        color = AltairTheme.Colors.textPrimary,
    )
    Spacer(modifier = Modifier.height(AltairTheme.Spacing.md))
}

@Composable
private fun TypographyShowcase() {
    Column {
        AltairText(text = "Display Large", style = AltairTheme.Typography.displayLarge)
        AltairText(text = "Display Medium", style = AltairTheme.Typography.displayMedium)
        AltairText(text = "Display Small", style = AltairTheme.Typography.displaySmall)
        Spacer(modifier = Modifier.height(AltairTheme.Spacing.sm))
        AltairText(text = "Headline Large", style = AltairTheme.Typography.headlineLarge)
        AltairText(text = "Headline Medium", style = AltairTheme.Typography.headlineMedium)
        AltairText(text = "Headline Small", style = AltairTheme.Typography.headlineSmall)
        Spacer(modifier = Modifier.height(AltairTheme.Spacing.sm))
        AltairText(text = "Body Large", style = AltairTheme.Typography.bodyLarge)
        AltairText(text = "Body Medium", style = AltairTheme.Typography.bodyMedium)
        AltairText(text = "Body Small", style = AltairTheme.Typography.bodySmall)
        Spacer(modifier = Modifier.height(AltairTheme.Spacing.sm))
        AltairText(text = "Label Large", style = AltairTheme.Typography.labelLarge)
        AltairText(text = "Label Medium", style = AltairTheme.Typography.labelMedium)
        AltairText(text = "Label Small", style = AltairTheme.Typography.labelSmall)
    }
}

@Composable
private fun ButtonShowcase() {
    Column {
        Row(horizontalArrangement = Arrangement.spacedBy(AltairTheme.Spacing.sm)) {
            AltairButton(onClick = {}, variant = ButtonVariant.Primary) {
                AltairText(text = "Primary", style = AltairTheme.Typography.labelLarge)
            }
            AltairButton(onClick = {}, variant = ButtonVariant.Secondary) {
                AltairText(text = "Secondary", style = AltairTheme.Typography.labelLarge)
            }
            AltairButton(onClick = {}, variant = ButtonVariant.Ghost) {
                AltairText(text = "Ghost", style = AltairTheme.Typography.labelLarge)
            }
            AltairButton(onClick = {}, variant = ButtonVariant.Danger) {
                AltairText(text = "Danger", style = AltairTheme.Typography.labelLarge)
            }
        }
        Spacer(modifier = Modifier.height(AltairTheme.Spacing.sm))
        Row(horizontalArrangement = Arrangement.spacedBy(AltairTheme.Spacing.sm)) {
            AltairButton(onClick = {}, variant = ButtonVariant.Primary, enabled = false) {
                AltairText(text = "Disabled", style = AltairTheme.Typography.labelLarge)
            }
            AltairButton(onClick = {}, variant = ButtonVariant.Primary) {
                AltairCircularProgressIndicator(size = 16.dp, strokeWidth = 2.dp)
                Spacer(modifier = Modifier.width(AltairTheme.Spacing.sm))
                AltairText(text = "Loading", style = AltairTheme.Typography.labelLarge)
            }
        }
    }
}

@Composable
private fun TextFieldShowcase() {
    var text1 by remember { mutableStateOf("") }
    var text2 by remember { mutableStateOf("test@example.com") }
    var text3 by remember { mutableStateOf("invalid") }

    Column(modifier = Modifier.fillMaxWidth()) {
        AltairTextField(
            value = text1,
            onValueChange = { text1 = it },
            label = "Empty Field",
            placeholder = "Enter text...",
            modifier = Modifier.fillMaxWidth(0.5f),
        )
        Spacer(modifier = Modifier.height(AltairTheme.Spacing.md))
        AltairTextField(
            value = text2,
            onValueChange = { text2 = it },
            label = "With Value",
            placeholder = "Email",
            modifier = Modifier.fillMaxWidth(0.5f),
        )
        Spacer(modifier = Modifier.height(AltairTheme.Spacing.md))
        AltairTextField(
            value = text3,
            onValueChange = { text3 = it },
            label = "Error State",
            placeholder = "Email",
            isError = true,
            errorMessage = "Please enter a valid email address",
            modifier = Modifier.fillMaxWidth(0.5f),
        )
        Spacer(modifier = Modifier.height(AltairTheme.Spacing.md))
        AltairTextField(
            value = "Disabled",
            onValueChange = {},
            label = "Disabled Field",
            enabled = false,
            modifier = Modifier.fillMaxWidth(0.5f),
        )
    }
}

@Composable
private fun CardShowcase() {
    Row(horizontalArrangement = Arrangement.spacedBy(AltairTheme.Spacing.md)) {
        AltairCard(elevation = CardElevation.None) {
            Column {
                AltairText(text = "No Elevation", style = AltairTheme.Typography.labelLarge)
                AltairText(
                    text = "Flat card with border",
                    style = AltairTheme.Typography.bodySmall,
                    color = AltairTheme.Colors.textSecondary,
                )
            }
        }
        AltairCard(elevation = CardElevation.Low) {
            Column {
                AltairText(text = "Low Elevation", style = AltairTheme.Typography.labelLarge)
                AltairText(
                    text = "Subtle shadow",
                    style = AltairTheme.Typography.bodySmall,
                    color = AltairTheme.Colors.textSecondary,
                )
            }
        }
        AltairCard(elevation = CardElevation.Medium) {
            Column {
                AltairText(text = "Medium Elevation", style = AltairTheme.Typography.labelLarge)
                AltairText(
                    text = "Moderate shadow",
                    style = AltairTheme.Typography.bodySmall,
                    color = AltairTheme.Colors.textSecondary,
                )
            }
        }
    }
}

@Composable
private fun CheckboxShowcase() {
    var checked1 by remember { mutableStateOf(false) }
    var checked2 by remember { mutableStateOf(true) }

    Column {
        AltairCheckboxRow(
            checked = checked1,
            onCheckedChange = { checked1 = it },
            label = "Unchecked option",
        )
        Spacer(modifier = Modifier.height(AltairTheme.Spacing.sm))
        AltairCheckboxRow(
            checked = checked2,
            onCheckedChange = { checked2 = it },
            label = "Checked option",
        )
        Spacer(modifier = Modifier.height(AltairTheme.Spacing.sm))
        AltairCheckboxRow(
            checked = false,
            onCheckedChange = {},
            label = "Disabled unchecked",
            enabled = false,
        )
        Spacer(modifier = Modifier.height(AltairTheme.Spacing.sm))
        AltairCheckboxRow(
            checked = true,
            onCheckedChange = {},
            label = "Disabled checked",
            enabled = false,
        )
    }
}

@Composable
private fun DialogShowcase() {
    var showDialog by remember { mutableStateOf(false) }
    var showConfirm by remember { mutableStateOf(false) }
    var showAlert by remember { mutableStateOf(false) }

    Row(horizontalArrangement = Arrangement.spacedBy(AltairTheme.Spacing.sm)) {
        AltairButton(onClick = { showDialog = true }, variant = ButtonVariant.Secondary) {
            AltairText(text = "Show Dialog", style = AltairTheme.Typography.labelLarge)
        }
        AltairButton(onClick = { showConfirm = true }, variant = ButtonVariant.Secondary) {
            AltairText(text = "Show Confirm", style = AltairTheme.Typography.labelLarge)
        }
        AltairButton(onClick = { showAlert = true }, variant = ButtonVariant.Secondary) {
            AltairText(text = "Show Alert", style = AltairTheme.Typography.labelLarge)
        }
    }

    AltairDialog(
        visible = showDialog,
        onDismissRequest = { showDialog = false },
        title = "Custom Dialog",
    ) {
        AltairText(
            text = "This is a custom dialog with arbitrary content.",
            style = AltairTheme.Typography.bodyMedium,
            color = AltairTheme.Colors.textSecondary,
        )
        Spacer(modifier = Modifier.height(AltairTheme.Spacing.md))
        AltairButton(onClick = { showDialog = false }) {
            AltairText(text = "Close", style = AltairTheme.Typography.labelLarge)
        }
    }

    AltairConfirmDialog(
        visible = showConfirm,
        onDismissRequest = { showConfirm = false },
        onConfirm = { /* Handle confirm */ },
        title = "Confirm Action",
        message = "Are you sure you want to proceed with this action?",
    )

    AltairAlertDialog(
        visible = showAlert,
        onDismissRequest = { showAlert = false },
        title = "Alert",
        message = "This is an informational alert message.",
    )
}

@Composable
private fun DropdownShowcase() {
    var expanded by remember { mutableStateOf(false) }

    Column {
        AltairButton(onClick = { expanded = true }, variant = ButtonVariant.Secondary) {
            AltairText(text = "Open Menu", style = AltairTheme.Typography.labelLarge)
        }

        AltairDropdownMenu(
            expanded = expanded,
            onDismissRequest = { expanded = false },
        ) {
            AltairDropdownMenuHeader("Actions")
            AltairDropdownMenuItem(text = "Option 1", onClick = { expanded = false })
            AltairDropdownMenuItem(text = "Option 2", onClick = { expanded = false })
            AltairDropdownMenuDivider()
            AltairDropdownMenuHeader("More")
            AltairDropdownMenuItem(text = "Option 3", onClick = { expanded = false })
            AltairDropdownMenuItem(text = "Disabled", onClick = {}, enabled = false)
        }
    }
}

@Composable
private fun ProgressShowcase() {
    Row(horizontalArrangement = Arrangement.spacedBy(AltairTheme.Spacing.md)) {
        Column {
            AltairText(text = "Default", style = AltairTheme.Typography.labelSmall)
            Spacer(modifier = Modifier.height(AltairTheme.Spacing.xs))
            AltairCircularProgressIndicator()
        }
        Column {
            AltairText(text = "Small", style = AltairTheme.Typography.labelSmall)
            Spacer(modifier = Modifier.height(AltairTheme.Spacing.xs))
            AltairCircularProgressIndicator(size = 16.dp, strokeWidth = 2.dp)
        }
        Column {
            AltairText(text = "Large", style = AltairTheme.Typography.labelSmall)
            Spacer(modifier = Modifier.height(AltairTheme.Spacing.xs))
            AltairCircularProgressIndicator(size = 40.dp, strokeWidth = 4.dp)
        }
        Column {
            AltairText(text = "Custom Color", style = AltairTheme.Typography.labelSmall)
            Spacer(modifier = Modifier.height(AltairTheme.Spacing.xs))
            AltairCircularProgressIndicator(color = AltairTheme.Colors.success)
        }
    }
}
