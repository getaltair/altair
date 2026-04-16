package com.getaltair.altair.ui.tracking

import androidx.compose.ui.test.junit4.createComposeRule
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performTextInput
import androidx.navigation.NavController
import androidx.test.ext.junit.runners.AndroidJUnit4
import com.getaltair.altair.ui.theme.AltairTheme
import io.mockk.every
import io.mockk.mockk
import io.mockk.verify
import kotlinx.coroutines.flow.MutableStateFlow
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class ItemCreationFormTest {
    @get:Rule
    val composeTestRule = createComposeRule()

    @Test
    fun submit_callsViewModelCreate() {
        val viewModel = mockk<TrackingViewModel>(relaxed = true)
        every { viewModel.locations } returns MutableStateFlow(emptyList())
        every { viewModel.categories } returns MutableStateFlow(emptyList())
        every { viewModel.uiState } returns MutableStateFlow(TrackingUiState.Idle)

        val navController = mockk<NavController>(relaxed = true)

        composeTestRule.setContent {
            AltairTheme {
                ItemCreationScreen(
                    navController = navController,
                    viewModel = viewModel,
                )
            }
        }

        composeTestRule.onNodeWithText("Name").performClick()
        composeTestRule.onNodeWithText("Name").performTextInput("Test Item")

        composeTestRule.onNodeWithText("Create Item").performClick()

        verify {
            viewModel.createItem(
                name = "Test Item",
                quantity = any(),
                locationId = any(),
                categoryId = any(),
                barcode = any(),
            )
        }
    }
}
