package net.ecsousa.unifivpn

import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.context.properties.ConfigurationPropertiesScan
import org.springframework.boot.runApplication

@SpringBootApplication
@ConfigurationPropertiesScan
class UnifiVpnApplication

fun main(args: Array<String>) {
    runApplication<UnifiVpnApplication>(*args)
}
