package net.ecsousa.unifivpn.config

import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Configuration
import org.springframework.http.codec.json.JacksonJsonDecoder
import org.springframework.web.reactive.function.client.ExchangeStrategies
import org.springframework.web.reactive.function.client.WebClient
import tools.jackson.core.StreamReadFeature
import tools.jackson.databind.json.JsonMapper

@Configuration
class WebClientConfig(
    private val appConfig: AppConfig,
) {

    @Bean
    fun webClientBuilder(): WebClient.Builder {
        val mapper = JsonMapper.builder()
            .enable(StreamReadFeature.INCLUDE_SOURCE_IN_LOCATION)
            .build();

        return WebClient.builder()
            .codecs {
                it.defaultCodecs().jacksonJsonDecoder(JacksonJsonDecoder(mapper))
                it.defaultCodecs().maxInMemorySize(16 * 1024 * 1024)
            }
    }

    @Bean
    fun mullvadClient(webClientBuilder: WebClient.Builder): WebClient {
        return webClientBuilder
            .baseUrl("https://api.mullvad.net/app/")
            .build()

    }

    @Bean
    fun unifiClient(webClientBuilder: WebClient.Builder): WebClient {
        return webClientBuilder
            .baseUrl(appConfig.unifiBaseUrl)
            .build();
    }


}