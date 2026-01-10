package com.getaltair.altair.ui

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.List
import androidx.compose.material.icons.filled.CheckCircle
import androidx.compose.material.icons.filled.Edit
import androidx.compose.material3.Icon
import androidx.compose.material3.NavigationBar
import androidx.compose.material3.NavigationBarItem
import androidx.compose.material3.NavigationBarItemDefaults
import androidx.compose.material3.NavigationRail
import androidx.compose.material3.NavigationRailItem
import androidx.compose.material3.NavigationRailItemDefaults
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import com.arkivanov.decompose.extensions.compose.stack.Children
import com.arkivanov.decompose.extensions.compose.stack.animation.fade
import com.arkivanov.decompose.extensions.compose.stack.animation.stackAnimation
import com.arkivanov.decompose.extensions.compose.subscribeAsState
import com.getaltair.altair.navigation.GuidanceComponent
import com.getaltair.altair.navigation.KnowledgeComponent
import com.getaltair.altair.navigation.RootComponent
import com.getaltair.altair.navigation.TrackingComponent
import com.getaltair.altair.ui.theme.AltairTheme

/**
 * Root content composable that renders the main application layout.
 *
 * Uses bottom navigation on mobile and sidebar navigation on desktop.
 *
 * @param component The RootComponent managing navigation state
 * @param useRail Whether to use NavigationRail (desktop) or NavigationBar (mobile)
 * @param modifier Modifier to be applied to the root layout
 */
@Composable
fun RootContent(
    component: RootComponent,
    useRail: Boolean = false,
    modifier: Modifier = Modifier,
) {
    val childStack by component.childStack.subscribeAsState()
    val activeChild = childStack.active.instance

    AltairTheme {
        val colors = AltairTheme.colors
        val typography = AltairTheme.typography

        if (useRail) {
            // Desktop layout with NavigationRail
            Row(modifier = modifier.fillMaxSize().background(colors.background)) {
                NavigationRail(
                    containerColor = colors.surface,
                    contentColor = colors.textPrimary,
                ) {
                    NavigationRailItem(
                        selected = activeChild is RootComponent.Child.Guidance,
                        onClick = component::onGuidanceClicked,
                        icon = {
                            Icon(
                                imageVector = Icons.Default.CheckCircle,
                                contentDescription = "Guidance",
                            )
                        },
                        label = {
                            Text(
                                text = "Guidance",
                                style = typography.labelSmall,
                            )
                        },
                        colors = NavigationRailItemDefaults.colors(
                            selectedIconColor = colors.accent,
                            selectedTextColor = colors.accent,
                            unselectedIconColor = colors.textSecondary,
                            unselectedTextColor = colors.textSecondary,
                            indicatorColor = colors.surfaceHover,
                        ),
                    )
                    NavigationRailItem(
                        selected = activeChild is RootComponent.Child.Knowledge,
                        onClick = component::onKnowledgeClicked,
                        icon = {
                            Icon(
                                imageVector = Icons.Default.Edit,
                                contentDescription = "Knowledge",
                            )
                        },
                        label = {
                            Text(
                                text = "Knowledge",
                                style = typography.labelSmall,
                            )
                        },
                        colors = NavigationRailItemDefaults.colors(
                            selectedIconColor = colors.accent,
                            selectedTextColor = colors.accent,
                            unselectedIconColor = colors.textSecondary,
                            unselectedTextColor = colors.textSecondary,
                            indicatorColor = colors.surfaceHover,
                        ),
                    )
                    NavigationRailItem(
                        selected = activeChild is RootComponent.Child.Tracking,
                        onClick = component::onTrackingClicked,
                        icon = {
                            Icon(
                                imageVector = Icons.AutoMirrored.Filled.List,
                                contentDescription = "Tracking",
                            )
                        },
                        label = {
                            Text(
                                text = "Tracking",
                                style = typography.labelSmall,
                            )
                        },
                        colors = NavigationRailItemDefaults.colors(
                            selectedIconColor = colors.accent,
                            selectedTextColor = colors.accent,
                            unselectedIconColor = colors.textSecondary,
                            unselectedTextColor = colors.textSecondary,
                            indicatorColor = colors.surfaceHover,
                        ),
                    )
                }

                Box(modifier = Modifier.weight(1f)) {
                    ChildContent(component = component)
                }
            }
        } else {
            // Mobile layout with bottom NavigationBar
            Scaffold(
                containerColor = colors.background,
                contentColor = colors.textPrimary,
                bottomBar = {
                    NavigationBar(
                        containerColor = colors.surface,
                        contentColor = colors.textPrimary,
                    ) {
                        NavigationBarItem(
                            selected = activeChild is RootComponent.Child.Guidance,
                            onClick = component::onGuidanceClicked,
                            icon = {
                                Icon(
                                    imageVector = Icons.Default.CheckCircle,
                                    contentDescription = "Guidance",
                                )
                            },
                            label = {
                                Text(
                                    text = "Guidance",
                                    style = typography.labelSmall,
                                )
                            },
                            colors = NavigationBarItemDefaults.colors(
                                selectedIconColor = colors.accent,
                                selectedTextColor = colors.accent,
                                unselectedIconColor = colors.textSecondary,
                                unselectedTextColor = colors.textSecondary,
                                indicatorColor = colors.surfaceHover,
                            ),
                        )
                        NavigationBarItem(
                            selected = activeChild is RootComponent.Child.Knowledge,
                            onClick = component::onKnowledgeClicked,
                            icon = {
                                Icon(
                                    imageVector = Icons.Default.Edit,
                                    contentDescription = "Knowledge",
                                )
                            },
                            label = {
                                Text(
                                    text = "Knowledge",
                                    style = typography.labelSmall,
                                )
                            },
                            colors = NavigationBarItemDefaults.colors(
                                selectedIconColor = colors.accent,
                                selectedTextColor = colors.accent,
                                unselectedIconColor = colors.textSecondary,
                                unselectedTextColor = colors.textSecondary,
                                indicatorColor = colors.surfaceHover,
                            ),
                        )
                        NavigationBarItem(
                            selected = activeChild is RootComponent.Child.Tracking,
                            onClick = component::onTrackingClicked,
                            icon = {
                                Icon(
                                    imageVector = Icons.AutoMirrored.Filled.List,
                                    contentDescription = "Tracking",
                                )
                            },
                            label = {
                                Text(
                                    text = "Tracking",
                                    style = typography.labelSmall,
                                )
                            },
                            colors = NavigationBarItemDefaults.colors(
                                selectedIconColor = colors.accent,
                                selectedTextColor = colors.accent,
                                unselectedIconColor = colors.textSecondary,
                                unselectedTextColor = colors.textSecondary,
                                indicatorColor = colors.surfaceHover,
                            ),
                        )
                    }
                },
            ) { padding ->
                Box(modifier = Modifier.padding(padding)) {
                    ChildContent(component = component)
                }
            }
        }
    }
}

@Composable
private fun ChildContent(
    component: RootComponent,
    modifier: Modifier = Modifier,
) {
    Children(
        stack = component.childStack,
        modifier = modifier.fillMaxSize(),
        animation = stackAnimation(fade()),
    ) { child ->
        when (val instance = child.instance) {
            is RootComponent.Child.Guidance -> GuidanceContent(instance.component)
            is RootComponent.Child.Knowledge -> KnowledgeContent(instance.component)
            is RootComponent.Child.Tracking -> TrackingContent(instance.component)
        }
    }
}

@Composable
private fun GuidanceContent(
    component: GuidanceComponent,
    modifier: Modifier = Modifier,
) {
    val colors = AltairTheme.colors
    val typography = AltairTheme.typography
    val spacing = AltairTheme.spacing

    Box(
        modifier = modifier
            .fillMaxSize()
            .background(colors.background)
            .padding(spacing.md),
    ) {
        Text(
            text = "Guidance Module - Quests, Epics, Energy",
            style = typography.headlineMedium,
            color = colors.textPrimary,
        )
    }
}

@Composable
private fun KnowledgeContent(
    component: KnowledgeComponent,
    modifier: Modifier = Modifier,
) {
    val colors = AltairTheme.colors
    val typography = AltairTheme.typography
    val spacing = AltairTheme.spacing

    Box(
        modifier = modifier
            .fillMaxSize()
            .background(colors.background)
            .padding(spacing.md),
    ) {
        Text(
            text = "Knowledge Module - Notes, Folders, Tags",
            style = typography.headlineMedium,
            color = colors.textPrimary,
        )
    }
}

@Composable
private fun TrackingContent(
    component: TrackingComponent,
    modifier: Modifier = Modifier,
) {
    val colors = AltairTheme.colors
    val typography = AltairTheme.typography
    val spacing = AltairTheme.spacing

    Box(
        modifier = modifier
            .fillMaxSize()
            .background(colors.background)
            .padding(spacing.md),
    ) {
        Text(
            text = "Tracking Module - Items, Locations, Containers",
            style = typography.headlineMedium,
            color = colors.textPrimary,
        )
    }
}
