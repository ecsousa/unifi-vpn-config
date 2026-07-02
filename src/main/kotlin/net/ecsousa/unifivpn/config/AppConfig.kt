package net.ecsousa.unifivpn.config

import org.springframework.boot.context.properties.ConfigurationProperties

@ConfigurationProperties(prefix = "unifi-vpn")
data class AppConfig(
    val unifiUsername: String,
    val unifiPassword: String,
    val unifiBaseUrl: String,
)