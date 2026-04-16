package com.getaltair.altair.data.auth

import android.util.Log
import okhttp3.Interceptor
import okhttp3.Response

private const val TAG = "AuthInterceptor"

class AuthInterceptor(
    private val tokenPreferences: TokenPreferences,
) : Interceptor {
    override fun intercept(chain: Interceptor.Chain): Response {
        val token = tokenPreferences.accessToken
        val request =
            if (token != null) {
                chain
                    .request()
                    .newBuilder()
                    .header("Authorization", "Bearer $token")
                    .build()
            } else {
                Log.d(TAG, "Sending request without access token to ${chain.request().url}")
                chain.request()
            }
        return chain.proceed(request)
    }
}
