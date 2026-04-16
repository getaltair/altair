package com.getaltair.altair.ui.theme

import androidx.compose.material3.Typography
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.googlefonts.Font
import androidx.compose.ui.text.googlefonts.GoogleFont
import androidx.compose.ui.unit.sp
import com.getaltair.altair.R

private val GoogleFontsProvider =
    GoogleFont.Provider(
        providerAuthority = "com.google.android.gms.fonts",
        providerPackage = "com.google.android.gms",
        certificates = R.array.com_google_android_gms_fonts_certs,
    )

val ManropeFamily =
    FontFamily(
        Font(googleFont = GoogleFont("Manrope"), fontProvider = GoogleFontsProvider, weight = FontWeight.Normal),
        Font(googleFont = GoogleFont("Manrope"), fontProvider = GoogleFontsProvider, weight = FontWeight.Medium),
        Font(googleFont = GoogleFont("Manrope"), fontProvider = GoogleFontsProvider, weight = FontWeight.SemiBold),
        Font(googleFont = GoogleFont("Manrope"), fontProvider = GoogleFontsProvider, weight = FontWeight.Bold),
    )

val PlusJakartaSansFamily =
    FontFamily(
        Font(googleFont = GoogleFont("Plus Jakarta Sans"), fontProvider = GoogleFontsProvider, weight = FontWeight.Normal),
        Font(googleFont = GoogleFont("Plus Jakarta Sans"), fontProvider = GoogleFontsProvider, weight = FontWeight.Medium),
        Font(googleFont = GoogleFont("Plus Jakarta Sans"), fontProvider = GoogleFontsProvider, weight = FontWeight.SemiBold),
    )

val Typography =
    Typography(
        displayLarge =
            TextStyle(
                fontFamily = ManropeFamily,
                fontWeight = FontWeight.Bold,
                fontSize = 57.sp,
                lineHeight = 64.sp,
                letterSpacing = (-0.25).sp,
            ),
        headlineLarge =
            TextStyle(
                fontFamily = ManropeFamily,
                fontWeight = FontWeight.SemiBold,
                fontSize = 32.sp,
                lineHeight = 40.sp,
                letterSpacing = 0.sp,
            ),
        bodyLarge =
            TextStyle(
                fontFamily = PlusJakartaSansFamily,
                fontWeight = FontWeight.Normal,
                fontSize = 16.sp,
                lineHeight = 24.sp,
                letterSpacing = 0.5.sp,
            ),
        bodyMedium =
            TextStyle(
                fontFamily = PlusJakartaSansFamily,
                fontWeight = FontWeight.Normal,
                fontSize = 14.sp,
                lineHeight = 20.sp,
                letterSpacing = 0.25.sp,
            ),
        labelMedium =
            TextStyle(
                fontFamily = PlusJakartaSansFamily,
                fontWeight = FontWeight.Medium,
                fontSize = 12.sp,
                lineHeight = 16.sp,
                letterSpacing = 0.5.sp,
            ),
        labelSmall =
            TextStyle(
                fontFamily = PlusJakartaSansFamily,
                fontWeight = FontWeight.Medium,
                fontSize = 11.sp,
                lineHeight = 16.sp,
                letterSpacing = 0.5.sp,
            ),
    )
