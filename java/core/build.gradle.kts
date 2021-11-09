import com.google.protobuf.gradle.protobuf
import com.google.protobuf.gradle.protoc
import org.gradle.plugins.ide.idea.model.IdeaModel

plugins {
    id("java")
    id("idea")
    id("antlr")
    id("com.google.protobuf") version "0.8.17"
}

dependencies {
    testImplementation("org.junit.jupiter:junit-jupiter-api:5.6.0")
    testImplementation("org.junit.jupiter:junit-jupiter-params:5.6.0")
    testRuntimeOnly("org.junit.jupiter:junit-jupiter-engine")
    implementation("com.google.protobuf:protobuf-java:3.17.3")
    implementation("com.fasterxml.jackson.core:jackson-databind:2.12.4")
    implementation("com.fasterxml.jackson.core:jackson-annotations:2.12.4")
    implementation("com.fasterxml.jackson.datatype:jackson-datatype-jdk8:2.12.4")
    implementation("com.fasterxml.jackson.dataformat:jackson-dataformat-yaml:2.12.4")
    implementation("com.google.code.findbugs:jsr305:3.0.2")


    antlr("org.antlr:antlr4:4.9.2")
    implementation("org.slf4j:slf4j-jdk14:1.7.30")
    implementation("org.antlr:antlr4:4.9.2")
    annotationProcessor("org.immutables:value:2.8.8")
    compileOnly("org.immutables:value-annotations:2.8.8")

}

java {
    toolchain {
        languageVersion.set(JavaLanguageVersion.of(17))
    }
}

sourceSets {
    main {
        proto.srcDir("../../proto")
        resources.srcDir("../../extensions")
        java.srcDir(file("build/generated/sources/antlr/main/java/"))
    }
}

project.configure<IdeaModel> {
    module {
        resourceDirs.addAll(listOf(
            file("../../text"),
            file("../../extensions"),
            file("../../proto")
        ))
        generatedSourceDirs.addAll(listOf(
            file("build/generated/sources/antlr/main"),
            file("build/generated/source/proto/main/java")
        ))
    }
}

tasks.named<AntlrTask>("generateGrammarSource") {
    arguments.add("-package")
    arguments.add("io.substrait.type")
    arguments.add("-visitor")
    arguments.add("-long-messages")
    arguments.add("-Xlog")
    arguments.add("-Werror")
    arguments.add("-Xexact-output-dir")
    setSource(fileTree("../../text/SubstraitType.g4"))
    outputDirectory = File(buildDir, "generated/sources/antlr/main/java/io/substrait/type")
}

protobuf {
    protoc {
        artifact = "com.google.protobuf:protoc:3.17.3"
    }
}
