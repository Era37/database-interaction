plugins {
    kotlin("jvm") version "1.7.21"
    application
}

group = "database.interaction"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    implementation("org.springframework:spring-context:5.3.9")
    implementation("org.springframework:spring-web:5.3.9")
    implementation("org.mongodb:mongodb-driver-sync:4.8.1")
}

tasks.jar {
    manifest {
        attributes("Main-Class" to "database.interaction.MainKt")
    }
}

application {
    mainClass.set("database.interaction.MainKt")
}