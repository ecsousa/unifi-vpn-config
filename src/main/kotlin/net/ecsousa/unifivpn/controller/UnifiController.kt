package net.ecsousa.unifivpn.controller

import net.ecsousa.unifivpn.model.SetVpnServerRequest
import net.ecsousa.unifivpn.model.unifi.NetworkConfig
import net.ecsousa.unifivpn.service.UnifiService
import org.springframework.http.ResponseEntity
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.PathVariable
import org.springframework.web.bind.annotation.PutMapping
import org.springframework.web.bind.annotation.RequestBody
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RestController
import tools.jackson.databind.JsonNode

@RestController
@RequestMapping("/api/unifi")
class UnifiController(
    private val unifiService: UnifiService
) {

    @GetMapping("/vpn-client/{id}")
    suspend fun getVpnClient(
        @PathVariable id: String,
    ): JsonNode? {
        return unifiService.getNetworkConfig(id)
    }

    @GetMapping("/vpn-clients")
    suspend fun getVpnClients(): List<JsonNode> {
        return unifiService.getNetworkConfigs()
    }

    @PutMapping("/vpn-client/{id}")
    suspend fun setVpnClientServer(
        @PathVariable id: String,
        @RequestBody requestBody: SetVpnServerRequest
    ) {
        unifiService.setVpnClientServer(id, requestBody.serverName)
    }


}