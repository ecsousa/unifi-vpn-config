package net.ecsousa.unifivpn.service

import kotlinx.coroutines.reactor.awaitSingle
import kotlinx.coroutines.reactor.awaitSingleOrNull
import kotlinx.coroutines.reactor.mono
import net.ecsousa.unifivpn.config.AppConfig
import net.ecsousa.unifivpn.exception.LoginFailedException
import net.ecsousa.unifivpn.exception.ResourceNotFoundException
import net.ecsousa.unifivpn.model.MullvadRelay
import net.ecsousa.unifivpn.model.unifi.LoginRequest
import net.ecsousa.unifivpn.model.unifi.UnifiEnvelope
import org.slf4j.LoggerFactory
import org.springframework.http.MediaType
import org.springframework.stereotype.Service
import org.springframework.web.reactive.function.client.WebClient
import org.springframework.web.reactive.function.client.bodyToMono
import tools.jackson.databind.JsonNode
import tools.jackson.databind.node.ObjectNode
import java.time.Duration

@Service
class UnifiService(
    appConfig: AppConfig,
    private val unifiClient: WebClient,
    private val mullvadService: MullvadService,
) {
    private val log = LoggerFactory.getLogger(UnifiService::class.java)

    private val cachedLoginRequest = unifiClient.post()
        .uri { it.path("api/auth/login").build() }
        .contentType(MediaType.APPLICATION_JSON)
        .bodyValue(LoginRequest(
            username = appConfig.unifiUsername,
            password = appConfig.unifiPassword,
        ))
        .exchangeToMono { response ->
            if(response.statusCode().is2xxSuccessful) {
                val csrf = response.headers().header("X-CSRF-Token").firstOrNull()

                response.releaseBody()
                    .thenReturn(response.cookies() to csrf)
            }
            else {
                mono {
                    throw LoginFailedException(
                        "unifi",
                        response.statusCode(),
                        response.bodyToMono<String>()
                            .awaitSingleOrNull()
                    )
                }
            }
        }
        .retryWhen(reactor.util.retry.Retry.backoff(3, Duration.ofSeconds(2)))
        .cache(
            { Duration.ofMinutes(5) },
            { Duration.ZERO },
            { Duration.ZERO }
        )

    suspend fun <S: WebClient.RequestHeadersSpec<S>> WebClient.RequestHeadersSpec<S>.unifiLogin(): S {
        val (cookies, csrf) = cachedLoginRequest.awaitSingle()

        return this
            .cookies { requestCookies ->
                cookies.values.flatten().forEach { cookie ->
                    requestCookies.add(cookie.name, cookie.value)
                }
            }
            .apply {
                csrf?.let {
                    header("X-CSRF-Token", it)
                }
            }
    }

    private fun <S: WebClient.RequestHeadersSpec<S>> WebClient.UriSpec<S>.getNetworkConfUri(id: String? = null): S {
        return this.uri { it.path("/proxy/network/api/s/default/rest/networkconf/${id ?: ""}").build() }
    }

    suspend fun getNetworkConfigs(): List<JsonNode> {

        return unifiClient
            .get()
            .getNetworkConfUri()
            .unifiLogin()
            .retrieve()
            .bodyToMono<UnifiEnvelope<JsonNode>>()
            .awaitSingle()
            .data
    }

    suspend fun getNetworkConfig(id: String): JsonNode {

        return unifiClient
            .get()
            .getNetworkConfUri(id)
            .unifiLogin()
            .retrieve()
            .bodyToMono<UnifiEnvelope<JsonNode>>()
            .awaitSingle()
            .data
            .firstOrNull()
            ?: throw ResourceNotFoundException("networkConfig", id)
    }

    suspend fun setVpnClientServer(id: String, serverName: String) {
        val relay = mullvadService.getServer(serverName)

        when(val node = getNetworkConfig(id)) {
            is ObjectNode -> {
                setVpnClientServer(id, node, relay)
            }

            else -> error("Unexpected json node type: ${node.javaClass.canonicalName}")
        }

    }

    private suspend fun setVpnClientServer(id: String, node: ObjectNode, relay: MullvadRelay) {
        node.put("wireguard_client_peer_ip", relay.hostname)
        node.put("wireguard_client_peer_public_key", relay.publicKey)

        unifiClient.put()
            .getNetworkConfUri(id)
            .unifiLogin()
            .contentType(MediaType.APPLICATION_JSON)
            .bodyValue(node)
            .retrieve()
            .toBodilessEntity()
            .doOnError { log.error("Failed to update VPN client server (id: {}). Payload: {}", id, node, it) }
            .awaitSingle()
    }

}