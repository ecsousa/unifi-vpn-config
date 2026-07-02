package net.ecsousa.unifivpn.model.mullvad

import com.fasterxml.jackson.annotation.JsonIgnoreProperties
import tools.jackson.databind.PropertyNamingStrategies
import tools.jackson.databind.annotation.JsonNaming

@JsonIgnoreProperties(ignoreUnknown = true)
@JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy::class)
data class MullvadRelaysResponse(
    val wireguard: MullvadWireguard,
)

@JsonIgnoreProperties(ignoreUnknown = true)
@JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy::class)
data class MullvadWireguard(
    val relays: List<MullvadWireguardRelay>,
)

@JsonIgnoreProperties(ignoreUnknown = true)
@JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy::class)
data class MullvadWireguardRelay(
    val hostname: String,
    val publicKey: String,
    val location: String,
)

