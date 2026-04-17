package com.getaltair.altair.ui.guidance

import android.Manifest
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Build
import android.util.Log
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.SnackbarHost
import androidx.compose.material3.SnackbarHostState
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.dp
import androidx.core.content.ContextCompat
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.LifecycleEventObserver
import androidx.lifecycle.compose.LocalLifecycleOwner
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.navigation.NavController
import com.getaltair.altair.service.FocusTimerService
import org.koin.androidx.compose.koinViewModel

private const val TAG = "FocusSessionScreen"

@Composable
fun FocusSessionScreen(
    questId: String,
    navController: NavController,
    viewModel: FocusSessionViewModel = koinViewModel(),
    modifier: Modifier = Modifier,
) {
    val context = LocalContext.current
    val lifecycleOwner = LocalLifecycleOwner.current

    LaunchedEffect(questId) {
        viewModel.init(questId)
        viewModel.start()
    }

    val remainingMs by viewModel.remainingMs.collectAsStateWithLifecycle()
    val isFinished by viewModel.isFinished.collectAsStateWithLifecycle()
    val errorMsg by viewModel.error.collectAsStateWithLifecycle()

    val snackbarHostState = remember { SnackbarHostState() }

    LaunchedEffect(isFinished) {
        if (isFinished) navController.popBackStack()
    }

    LaunchedEffect(errorMsg) {
        errorMsg?.let { snackbarHostState.showSnackbar(it) }
    }

    // Hoisted so the permission callback can access it after the dialog returns
    var pendingEndTimeMs by remember { mutableStateOf(0L) }

    val postNotifLauncher =
        rememberLauncherForActivityResult(
            ActivityResultContracts.RequestPermission(),
        ) { granted ->
            if (granted) {
                startFocusService(context, pendingEndTimeMs)
            } else {
                Log.w(TAG, "POST_NOTIFICATIONS denied — timer will run silently in background")
            }
        }

    DisposableEffect(lifecycleOwner) {
        val observer =
            LifecycleEventObserver { _, event ->
                when (event) {
                    Lifecycle.Event.ON_STOP -> {
                        // Read directly from ViewModel to avoid stale closure values
                        if (viewModel.isRunning.value) {
                            val endTimeMs = System.currentTimeMillis() + viewModel.remainingMs.value
                            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
                                if (ContextCompat.checkSelfPermission(
                                        context,
                                        Manifest.permission.POST_NOTIFICATIONS,
                                    ) == PackageManager.PERMISSION_GRANTED
                                ) {
                                    startFocusService(context, endTimeMs)
                                } else {
                                    pendingEndTimeMs = endTimeMs
                                    postNotifLauncher.launch(Manifest.permission.POST_NOTIFICATIONS)
                                }
                            } else {
                                startFocusService(context, endTimeMs)
                            }
                        }
                    }

                    Lifecycle.Event.ON_START -> {
                        context.stopService(Intent(context, FocusTimerService::class.java))
                    }

                    else -> {}
                }
            }
        lifecycleOwner.lifecycle.addObserver(observer)
        onDispose { lifecycleOwner.lifecycle.removeObserver(observer) }
    }

    val minutes = remainingMs / 60_000
    val seconds = (remainingMs % 60_000) / 1_000
    val timerText = "%02d:%02d".format(minutes, seconds)

    Scaffold(
        modifier = modifier,
        snackbarHost = { SnackbarHost(snackbarHostState) },
    ) { paddingValues ->
        Column(
            modifier =
                Modifier
                    .fillMaxSize()
                    .background(MaterialTheme.colorScheme.surfaceContainerLow)
                    .padding(paddingValues)
                    .padding(32.dp),
            verticalArrangement = Arrangement.Center,
            horizontalAlignment = Alignment.CenterHorizontally,
        ) {
            Text(
                text = "Focus",
                style = MaterialTheme.typography.titleLarge,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )

            Spacer(Modifier.height(32.dp))

            Text(
                text = timerText,
                style = MaterialTheme.typography.displayLarge,
                color = MaterialTheme.colorScheme.onSurface,
            )

            Spacer(Modifier.height(64.dp))

            Button(
                onClick = {
                    viewModel.end()
                    context.stopService(Intent(context, FocusTimerService::class.java))
                    navController.popBackStack()
                },
                shape = RoundedCornerShape(50),
                colors =
                    ButtonDefaults.buttonColors(
                        containerColor = MaterialTheme.colorScheme.errorContainer,
                        contentColor = MaterialTheme.colorScheme.onErrorContainer,
                    ),
            ) {
                Text("End Session")
            }
        }
    }
}

private fun startFocusService(
    context: android.content.Context,
    endTimeEpochMs: Long,
) {
    val intent =
        Intent(context, FocusTimerService::class.java).apply {
            putExtra(FocusTimerService.EXTRA_END_TIME_EPOCH_MS, endTimeEpochMs)
        }
    try {
        context.startForegroundService(intent)
    } catch (e: Exception) {
        Log.e(TAG, "Failed to start FocusTimerService", e)
    }
}
