package net.ecsousa.unifivpn.model.unifi

import com.fasterxml.jackson.annotation.JsonIgnoreProperties
import tools.jackson.databind.PropertyNamingStrategies
import tools.jackson.databind.annotation.JsonNaming

@JsonIgnoreProperties(ignoreUnknown = true)
@JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy::class)
data class UnifiEnvelope<out T>(
    val meta: UnifiEnvelopeMetadata,
    val data: List<T>,
)

@JsonIgnoreProperties(ignoreUnknown = true)
@JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy::class)
data class UnifiEnvelopeMetadata(
    val rc: String,
)