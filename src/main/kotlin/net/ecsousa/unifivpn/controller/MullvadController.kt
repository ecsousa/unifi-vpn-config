package net.ecsousa.unifivpn.controller

import net.ecsousa.unifivpn.model.MullvadRelay
import net.ecsousa.unifivpn.service.MullvadService
import org.springframework.http.ResponseEntity
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.PathVariable
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RestController

@RestController
@RequestMapping("/api/mullvad")
class MullvadController(
    private val mullvadService: MullvadService,
) {

    @GetMapping("/servers")
    suspend fun getServers(): List<MullvadRelay> {
        return mullvadService.getServerList()
    }

    @GetMapping("/server/{hostname}")
    suspend fun getServers(
        @PathVariable hostname: String,
    ): ResponseEntity<MullvadRelay> {
        return mullvadService.getServer(hostname)
            ?.let { ResponseEntity.ok(it) }
            ?: ResponseEntity.notFound().build()

    }



}