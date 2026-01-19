package com.getaltair.altair.ui.components

import androidx.compose.foundation.text.BasicText
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import com.getaltair.altair.ui.theme.AltairTheme

/**
 * Altair-styled text component.
 *
 * A simple text component that applies Altair theme typography
 * without Material Design dependencies.
 *
 * @param text The text to display.
 * @param modifier Modifier to be applied to the text.
 * @param style Typography style to apply. Defaults to [AltairTheme.Typography.bodyMedium].
 * @param color Text color. If not specified, uses the color from [style].
 * @param textAlign Text alignment within the available space.
 * @param overflow How to handle text overflow.
 * @param maxLines Maximum number of lines to display.
 */
@Composable
fun AltairText(
    text: String,
    modifier: Modifier = Modifier,
    style: TextStyle = AltairTheme.Typography.bodyMedium,
    color: Color = style.color,
    textAlign: TextAlign? = null,
    overflow: TextOverflow = TextOverflow.Clip,
    maxLines: Int = Int.MAX_VALUE,
) {
    BasicText(
        text = text,
        modifier = modifier,
        style = style.copy(
            color = color,
            textAlign = textAlign ?: style.textAlign,
        ),
        overflow = overflow,
        maxLines = maxLines,
    )
}
