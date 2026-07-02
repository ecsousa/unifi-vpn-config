package net.ecsousa.unifivpn.exception

import org.springframework.http.HttpStatusCode

class LoginFailedException(
    provider: String,
    statusCode: HttpStatusCode,
    responseBody: String?,
): RuntimeException("Login failed at $provider (code $statusCode)${responseBody?.let {" with message: $it"}}.")