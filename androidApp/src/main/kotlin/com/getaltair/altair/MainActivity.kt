package com.getaltair.altair

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.runtime.Composable
import androidx.compose.ui.tooling.preview.Preview
import com.arkivanov.decompose.DefaultComponentContext
import com.arkivanov.decompose.defaultComponentContext
import com.arkivanov.essenty.lifecycle.LifecycleRegistry
import com.getaltair.altair.di.initKoinAndroid
import com.getaltair.altair.navigation.HomeComponent
import com.getaltair.altair.navigation.RootComponent
import com.getaltair.altair.ui.HomeScreen

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        // Initialize Koin with Android context (only once)
        if (savedInstanceState == null) {
            initKoinAndroid(applicationContext)
        }

        enableEdgeToEdge()
        super.onCreate(savedInstanceState)

        // Create root component with activity's component context
        val rootComponent = RootComponent(
            componentContext = defaultComponentContext()
        )

        setContent {
            App(rootComponent)
        }
    }
}

/**
 * Preview composable for Android Studio.
 * Shows HomeScreen for preview purposes since App() requires RootComponent.
 * Uses HomeComponent with default no-op onOutput callback.
 */
@Preview
@Composable
fun AppAndroidPreview() {
    // Create a preview-safe ComponentContext using LifecycleRegistry
    val previewContext = DefaultComponentContext(lifecycle = LifecycleRegistry())

    // HomeComponent accepts default no-op onOutput, so we can instantiate directly
    HomeScreen(
        component = HomeComponent(
            componentContext = previewContext,
            onOutput = {} // explicit no-op for clarity
        )
    )
}
