# Stage 1: Build Backend
FROM eclipse-temurin:25-jdk AS backend-builder
WORKDIR /app

COPY ./ ./
RUN chmod +x gradlew
RUN ./gradlew copyAgent
ARG APP_VERSION="0.1"
RUN ./gradlew build -x test -PreleaseVersion=$APP_VERSION

# Stage 2: Final Image
FROM eclipse-temurin:25-jre
WORKDIR /app
COPY --from=backend-builder /app/build/agent/*.jar /app/reactor-tools.jar
ARG APP_VERSION="0.1"
COPY --from=backend-builder /app/build/libs/unifi-vpn-config-${APP_VERSION}.jar app.jar

ENV PORT=8080
ENV UNIFI_USERNAME=""
ENV UNIFI_PASSWORD=""
ENV UNIFI_BASEURL=""

EXPOSE 8080
ENTRYPOINT ["java", "-javaagent:/app/reactor-tools.jar", "-jar", "app.jar"]
