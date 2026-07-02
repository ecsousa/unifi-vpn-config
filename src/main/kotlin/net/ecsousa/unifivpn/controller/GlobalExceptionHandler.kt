package net.ecsousa.unifivpn.controller

import net.ecsousa.unifivpn.exception.LoginFailedException
import net.ecsousa.unifivpn.exception.ResourceNotFoundException
import net.ecsousa.unifivpn.model.ErrorResponse
import org.springframework.http.HttpStatus
import org.springframework.http.ResponseEntity
import org.springframework.web.bind.annotation.ExceptionHandler
import org.springframework.web.bind.annotation.RestControllerAdvice

@RestControllerAdvice
class GlobalExceptionHandler {

    @ExceptionHandler(ResourceNotFoundException::class)
    fun handleResourceNotFound(ex: ResourceNotFoundException): ResponseEntity<ErrorResponse> {
        return ResponseEntity.status(HttpStatus.NOT_FOUND)
            .body(ErrorResponse("notFound", ex.message ?: ""))
    }

    @ExceptionHandler(LoginFailedException::class)
    fun handleLoginFailedException(ex: LoginFailedException): ResponseEntity<ErrorResponse> {
        return ResponseEntity.status(HttpStatus.NOT_FOUND)
            .body(ErrorResponse("loginFailed", ex.message ?: ""))
    }

}