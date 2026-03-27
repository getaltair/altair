package com.getaltair.altair.ui.theme

import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.darkColorScheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.Composable

private val LightColorScheme = lightColorScheme(
    primary = DeepMutedTealNavy,
    onPrimary = OnPrimaryLight,
    primaryContainer = WhisperSoftIceBlue,
    onPrimaryContainer = SubduedTealNavy,
    secondary = SlateHarbor,
    onSecondary = OnPrimaryLight,
    secondaryContainer = MorningFrostBlue,
    onSecondaryContainer = SlateHarbor,
    tertiary = WeatheredStoneGreen,
    onTertiary = OnPrimaryLight,
    tertiaryContainer = MistySageWash,
    onTertiaryContainer = WeatheredStoneGreen,
    error = WarmTerracotta,
    onError = OnPrimaryLight,
    errorContainer = SoftCoralBlush,
    onErrorContainer = WarmTerracotta,
    background = FrostTouchedPearl,
    onBackground = DeepInkCharcoal,
    surface = FrostTouchedPearl,
    onSurface = DeepInkCharcoal,
    surfaceVariant = PaleMorningMist,
    onSurfaceVariant = MutedSlate,
    surfaceContainerLowest = PureWhite,
    surfaceContainerLow = PaleMorningMist,
    surfaceContainer = SoftCloudGrey,
    surfaceContainerHigh = CoolPebbleWash,
    surfaceContainerHighest = GentleStone,
    surfaceDim = VeiledSeafoam,
    outline = WornGraphite,
    outlineVariant = SilverHaze,
    inverseSurface = DeepInkCharcoal,
    inverseOnSurface = FrostTouchedPearl,
    inversePrimary = GentleDawnBlue,
)

private val DarkColorScheme = darkColorScheme(
    primary = GentleDawnBlue,
    onPrimary = SubduedTealNavy,
    primaryContainer = SubduedTealNavy,
    onPrimaryContainer = WhisperSoftIceBlue,
    secondary = MorningFrostBlue,
    onSecondary = SubduedTealNavy,
    secondaryContainer = SlateHarbor,
    onSecondaryContainer = MorningFrostBlue,
    tertiary = MistySageWash,
    onTertiary = SubduedTealNavy,
    tertiaryContainer = WeatheredStoneGreen,
    onTertiaryContainer = MistySageWash,
    error = SoftCoralBlush,
    onError = WarmTerracotta,
    errorContainer = WarmTerracotta,
    onErrorContainer = SoftCoralBlush,
    background = DarkSurface,
    onBackground = DarkOnSurface,
    surface = DarkSurface,
    onSurface = DarkOnSurface,
    surfaceVariant = DarkSurfaceContainerHigh,
    onSurfaceVariant = DarkOnSurfaceVariant,
    surfaceContainerLowest = DarkSurfaceDim,
    surfaceContainerLow = DarkSurfaceContainerLow,
    surfaceContainer = DarkSurfaceContainer,
    surfaceContainerHigh = DarkSurfaceContainerHigh,
    surfaceContainerHighest = DarkSurfaceContainerHighest,
    surfaceDim = DarkSurfaceDim,
    outline = DarkOutline,
    outlineVariant = DarkOutlineVariant,
    inverseSurface = FrostTouchedPearl,
    inverseOnSurface = DeepInkCharcoal,
    inversePrimary = DeepMutedTealNavy,
)

@Composable
fun AltairTheme(
    darkTheme: Boolean = isSystemInDarkTheme(),
    content: @Composable () -> Unit,
) {
    val colorScheme = if (darkTheme) DarkColorScheme else LightColorScheme

    MaterialTheme(
        colorScheme = colorScheme,
        typography = AltairTypography,
        shapes = AltairShapes,
        content = content,
    )
}
