package net.ecsousa.unifivpn.model

import net.ecsousa.unifivpn.exception.ResourceNotFoundException

data class ErrorResponse(
    val type: String,
    val message: String,
)
