plugins {
    id("org.springframework.boot") version "4.1.0"
    kotlin("jvm") version "2.3.21"
    kotlin("plugin.spring") version "2.3.21"
}

group = "com.stmgr"
version = if (project.hasProperty("releaseVersion")) project.property("releaseVersion")!! else "0.1"

java {
    toolchain {
        languageVersion.set(JavaLanguageVersion.of(25))
    }
}

repositories {
    mavenCentral()
}

val reactorAgent by configurations.creating

dependencies {
    implementation(enforcedPlatform("org.springframework.boot:spring-boot-dependencies:4.1.0"))
    reactorAgent(enforcedPlatform("org.springframework.boot:spring-boot-dependencies:4.1.0"))

    implementation("org.springframework.boot:spring-boot-starter-webflux")
    reactorAgent("io.projectreactor:reactor-tools") {
        isTransitive = false
    }

    // Kotlin support (required by Spring for suspend function invocation and JSON serialization)
    implementation("org.jetbrains.kotlin:kotlin-reflect")
    implementation("tools.jackson.module:jackson-module-kotlin")

    // Kotlin coroutines
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-reactor")
    implementation("io.projectreactor.kotlin:reactor-kotlin-extensions")

    testImplementation("org.springframework.boot:spring-boot-starter-test")
    testImplementation("org.jetbrains.kotlin:kotlin-test-junit5")
    testImplementation("org.jetbrains.kotlinx:kotlinx-coroutines-test")
}

tasks.named<Test>("test") {
    useJUnitPlatform()
}

tasks.register<Copy>("copyAgent") {
    from(reactorAgent)
    into(layout.buildDirectory.dir("agent"))
}

tasks.named<org.springframework.boot.gradle.tasks.run.BootRun>("bootRun") {
    dependsOn("copyAgent")
    doFirst {
        val agentJar = reactorAgent.singleFile
        jvmArgs("-javaagent:${agentJar.absolutePath}")
    }
}
