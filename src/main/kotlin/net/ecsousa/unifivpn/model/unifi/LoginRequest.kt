package net.ecsousa.unifivpn.model.unifi

data class LoginRequest(
    val username: String,
    val password: String,
    val rememberMe: Boolean = false,
    val token: String = "",
)