package net.ecsousa.unifivpn.service

import kotlinx.coroutines.reactor.awaitSingle
import net.ecsousa.unifivpn.model.MullvadRelay
import net.ecsousa.unifivpn.model.mullvad.MullvadRelaysResponse
import org.slf4j.LoggerFactory
import org.springframework.stereotype.Service
import org.springframework.web.reactive.function.client.WebClient
import org.springframework.web.reactive.function.client.bodyToMono
import java.time.Duration

@Service
class MullvadService(
    mullvadClient: WebClient,
) {
    private val log = LoggerFactory.getLogger(MullvadService::class.java)

    private val cachedRelaysMono = mullvadClient.get()
        .uri { it
            .path("/v1/relays")
            .build()
        }
        .retrieve()
        .bodyToMono<MullvadRelaysResponse>()
        .retryWhen(reactor.util.retry.Retry.backoff(3, Duration.ofSeconds(2)))
        .doOnSuccess { log.info("Fetched Mullvad relays") }
        .cache(
            { Duration.ofMinutes(10) },
            { Duration.ZERO },
            { Duration.ZERO }
        )

    suspend fun getServerList(): List<MullvadRelay> {
        return cachedRelaysMono
            .awaitSingle()
            .wireguard.relays
            .map {
                MullvadRelay(
                    hostname = "${it.hostname}.relays.mullvad.net",
                    publicKey = it.publicKey,
                    location = it.location,
                )
            }
    }

    suspend fun getServer(name: String): MullvadRelay? {
        return cachedRelaysMono
            .awaitSingle()
            .wireguard.relays
            .firstOrNull() { it.hostname == name }
            ?.let {
                MullvadRelay(
                    hostname = "${it.hostname}.relays.mullvad.net",
                    publicKey = it.publicKey,
                    location = it.location,
                )
            }
    }

}