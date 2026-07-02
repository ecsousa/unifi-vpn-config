# UniFi VPN Config

A Spring Boot (Kotlin WebFlux) application to automate and manage UniFi VPN Client configurations using Mullvad VPN relays.

## Features

- **Automated VPN Client Setup**: Integrates directly with your UniFi controller's API to update VPN client settings dynamically.
- **Mullvad Integration**: Automatically fetches and applies the latest Mullvad relay IP addresses and public keys.
- **Resilient Connections**: Implements exponential backoff retries and avoids caching transient network errors to ensure reliable operation.

## Configuration

The application requires the following environment variables to communicate with your UniFi controller:

- `UNIFI_USERNAME`: Your UniFi local administrator username.
- `UNIFI_PASSWORD`: Your UniFi local administrator password.
- `UNIFI_BASEURL`: The base URL to your UniFi controller (e.g., `https://192.168.1.1` or `https://unifi.yourdomain.com`).
- `PORT`: The port on which the application server runs (default: 8080).

## Local Development

1. Clone the repository.
2. Ensure you have Java 25 installed.
3. Provide your environment variables (either in your IDE or via terminal).
4. Run using Gradle:
   ```bash
   ./gradlew bootRun
   ```

## Building and Running the JAR Locally

You can build a single executable JAR file containing the application.

### Build Steps

1. Use Gradle to build the project, skipping tests:
   ```bash
   ./gradlew build -x test
   ```
   The generated JAR file will be located at `build/libs/unifi-vpn-config-0.1.jar`.

### Running the JAR

Run the assembled JAR using the `java -jar` command, specifying the required environment variables:

```bash
UNIFI_USERNAME="your-username" UNIFI_PASSWORD="your-password" UNIFI_BASEURL="https://192.168.1.1" java -jar build/libs/unifi-vpn-config-0.1.jar
```

The application will be accessible at `http://localhost:8080`.

## Docker Deployment

Build the complete application image using the provided Dockerfile:
```bash
docker build -t unifi-vpn-config .
```

### Using Docker Compose (Recommended)

Create a `docker-compose.yml` file to easily run the container as a background service:

```yaml
version: '3.8'
services:
  unifi-vpn-config:
    image: ghcr.io/ecsousa/unifi-vpn-config:main
    ports:
      - "8080:8080"
    environment:
      - UNIFI_USERNAME=your-username
      - UNIFI_PASSWORD=your-password
      - UNIFI_BASEURL=https://192.168.1.1
    restart: unless-stopped
```

Start the container with:
```bash
docker-compose up -d
```

## GitHub Actions

This repository is equipped with fully automated CI/CD pipelines:

- **Build and Push (Main)**: Pushes to the `main` branch automatically build a multi-architecture Docker image and publish it to the GitHub Container Registry (`ghcr.io`) tagged as `:main`.
- **Releases**: Creating a new GitHub Release (or pushing a `v*` tag) automatically:
  - Compiles the application and attaches the standalone `.jar` file to the GitHub Release.
  - Builds and pushes the Docker image to GHCR tagged with `:latest` as well as the specific semantic version numbers.
