package com.getaltair.altair.data.auth

import okhttp3.Interceptor
import okhttp3.Response

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
                chain.request()
            }
        return chain.proceed(request)
    }
}
