plugins {
    kotlin("jvm") version "1.7.21"
    application
}

group = "database.interaction"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

tasks.jar {
    manifest {
        attributes("Main-Class" to "database.interaction.MainKt")
    }
}

application {
    mainClass.set("database.interaction.MainKt")
}