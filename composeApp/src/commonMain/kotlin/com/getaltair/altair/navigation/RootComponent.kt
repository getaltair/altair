package com.getaltair.altair.navigation

import com.arkivanov.decompose.ComponentContext
import com.arkivanov.decompose.router.stack.ChildStack
import com.arkivanov.decompose.router.stack.StackNavigation
import com.arkivanov.decompose.router.stack.childStack
import com.arkivanov.decompose.router.stack.replaceAll
import com.arkivanov.decompose.value.Value
import com.arkivanov.essenty.lifecycle.coroutines.coroutineScope
import com.getaltair.altair.service.auth.AuthManager
import com.getaltair.altair.service.auth.AuthState
import com.getaltair.altair.ui.auth.LoginComponent
import com.getaltair.altair.ui.auth.RegisterComponent
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.flow.launchIn
import kotlinx.coroutines.flow.onEach
import kotlinx.coroutines.launch

/**
 * Root navigation component using Decompose.
 * Manages the navigation stack for the entire application.
 *
 * Navigation structure:
 * - Unauthenticated: Login/Register flow
 * - Authenticated: Main shell with tab navigation (Home, Guidance, Knowledge, Tracking, Settings)
 *
 * Handles auth state changes to redirect between authenticated
 * and unauthenticated flows.
 */
class RootComponent(
    componentContext: ComponentContext,
    private val authManager: AuthManager,
) : ComponentContext by componentContext {
    // Scope is automatically cancelled when the component is destroyed
    private val scope = coroutineScope(Dispatchers.Main.immediate + SupervisorJob())
    private val navigation = StackNavigation<Config>()

    val stack: Value<ChildStack<Config, Child>> =
        childStack(
            source = navigation,
            serializer = Config.serializer(),
            initialConfiguration = Config.Login, // Start at login, will redirect if authenticated
            handleBackButton = true,
            childFactory = ::child,
        )

    init {
        // Initialize auth manager and observe auth state
        scope.launch {
            authManager.initialize()
        }

        // React to auth state changes
        authManager.authState
            .onEach { state ->
                when (state) {
                    is AuthState.Authenticated -> {
                        // Navigate to main shell when authenticated
                        navigation.replaceAll(Config.Main)
                    }
                    is AuthState.Unauthenticated -> {
                        // Navigate to login when unauthenticated
                        navigation.replaceAll(Config.Login)
                    }
                    is AuthState.Loading -> {
                        // Stay on current screen while loading
                    }
                }
            }.launchIn(scope)
    }

    private fun child(
        config: Config,
        componentContext: ComponentContext,
    ): Child =
        when (config) {
            is Config.Main ->
                Child.Main(
                    MainComponent(
                        componentContext = componentContext,
                    ),
                )

            is Config.Login ->
                Child.Login(
                    LoginComponent(
                        componentContext = componentContext,
                        authManager = authManager,
                        onLoginSuccess = {
                            navigation.replaceAll(Config.Main)
                        },
                        onNavigateToRegister = {
                            navigation.replaceAll(Config.Register)
                        },
                    ),
                )

            is Config.Register ->
                Child.Register(
                    RegisterComponent(
                        componentContext = componentContext,
                        authManager = authManager,
                        onRegisterSuccess = {
                            navigation.replaceAll(Config.Main)
                        },
                        onNavigateToLogin = {
                            navigation.replaceAll(Config.Login)
                        },
                    ),
                )
        }

    sealed class Child {
        data class Main(
            val component: MainComponent,
        ) : Child()

        data class Login(
            val component: LoginComponent,
        ) : Child()

        data class Register(
            val component: RegisterComponent,
        ) : Child()
    }
}
