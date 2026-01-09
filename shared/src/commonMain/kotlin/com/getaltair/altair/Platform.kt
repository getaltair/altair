package com.getaltair.altair

interface Platform {
    val name: String
}

expect fun getPlatform(): Platform