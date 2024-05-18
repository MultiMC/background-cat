#![deny(dead_code)]

pub mod responses;
use responses::RESPONSES;

use lazy_static::lazy_static;
use regex::Regex;

pub fn common_mistakes(input: &str) -> Vec<(&str, String)> {
    PARSERS.iter().flat_map(|m| m(input)).collect()
}

pub(crate) type Check = fn(&str) -> Option<(&str, String)>;

pub(crate) const PARSERS: [Check; 17] = [
    multimc_in_program_files,
    macos_too_new_java,
    multimc_in_onedrive_managed_folder,
    forge_too_new_java,
    one_seventeen_plus_java_too_old,
    two_one_plus_java_too_old,
    m1_failed_to_find_service_port,
    pixel_format_not_accelerated_win10,
    intel_graphics_icd_dll,
    id_range_exceeded,
    out_of_memory_error,
    shadermod_optifine_conflict,
    fabric_api_missing,
    java_architecture,
    detect_temp_directories,
    using_system_glfw,
    using_system_openal,
];

fn multimc_in_program_files(log: &str) -> Option<(&str, String)> {
    const TRIGGER: &str = "Minecraft folder is:\nC:/Program Files";
    if log.contains(TRIGGER) {
        Some(("‼", RESPONSES.get("program-files")?.to_string()))
    } else {
        None
    }
}

fn macos_too_new_java(log: &str) -> Option<(&str, String)> {
    const TRIGGER: &str = r#"Terminating app due to uncaught exception 'NSInternalInconsistencyException', reason: 'NSWindow drag regions should only be invalidated on the Main Thread!'"#;
    if log.contains(TRIGGER) {
        Some(("‼", RESPONSES.get("macos-java-too-new")?.to_string()))
    } else {
        None
    }
}

fn id_range_exceeded(log: &str) -> Option<(&str, String)> {
    const TRIGGER: &str =
        "java.lang.RuntimeException: Invalid id 4096 - maximum id range exceeded.";
    if log.contains(TRIGGER) {
        Some(("‼", RESPONSES.get("id-limit")?.to_string()))
    } else {
        None
    }
}

fn out_of_memory_error(log: &str) -> Option<(&str, String)> {
    const TRIGGER: &str = "java.lang.OutOfMemoryError";
    if log.contains(TRIGGER) {
        Some(("‼", RESPONSES.get("out-of-memory")?.to_string()))
    } else {
        None
    }
}

fn shadermod_optifine_conflict(log: &str) -> Option<(&str, String)> {
    const TRIGGER: &str = "java.lang.RuntimeException: Shaders Mod detected. Please remove it, OptiFine has built-in support for shaders.";
    if log.contains(TRIGGER) {
        Some(("‼", RESPONSES.get("optifine-and-shadermod")?.to_string()))
    } else {
        None
    }
}

fn fabric_api_missing(log: &str) -> Option<(&str, String)> {
    const EXCEPTION: &str =
        "net.fabricmc.loader.discovery.ModResolutionException: Could not find required mod:";
    const FABRIC: &str = "requires {fabric @";

    if log.contains(EXCEPTION) && log.contains(FABRIC) {
        Some(("‼", RESPONSES.get("missing-fabric-api")?.to_string()))
    } else {
        None
    }
}

fn multimc_in_onedrive_managed_folder(log: &str) -> Option<(&str, String)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"Minecraft folder is:\nC:/.+/.+/OneDrive").unwrap();
    }
    if RE.is_match(log) {
        Some(("❗", RESPONSES.get("multimc-in-onedrive")?.to_string()))
    } else {
        None
    }
}

fn forge_too_new_java(log: &str) -> Option<(&str, String)> {
    const URLCLASSLOADER_CAST: &str = "java.lang.ClassCastException: class jdk.internal.loader.ClassLoaders$AppClassLoader cannot be cast to class java.net.URLClassLoader";
    if log.contains(URLCLASSLOADER_CAST) {
        Some(("‼", RESPONSES.get("use-java-8")?.to_string()))
    } else {
        None
    }
}

fn one_seventeen_plus_java_too_old(log: &str) -> Option<(&str, String)> {
    const FABRIC_JAVA_VERSION_ERROR: &str = "fabric requires {java @ [>=16]}";
    const FABRIC_JAVA_VERSION_ERROR_SEVENTEEN: &str = "fabric requires {java @ [>=17]}";
    const JAVA_17_WARNING: &str = "Minecraft 1.18 Pre Release 2 and above require the use of Java 17";

    if log.contains(FABRIC_JAVA_VERSION_ERROR)
        || log.contains(FABRIC_JAVA_VERSION_ERROR_SEVENTEEN)
        || log.contains(JAVA_17_WARNING)
    {
        Some(("‼", RESPONSES.get("use-java-17")?.to_string()))
    } else {
        None
    }
}

fn two_one_plus_java_too_old(log: &str) -> Option<(&str, String)> {
    const JAVA_CHECK_CLASS_FILE_VERSION: &str = "(class file version 65.0)";
    const JAVA_CHECK_CLASS_FILE_VERSION_MMC: &str = "Minecraft 24w14a and above require the use of Java 21";
    if log.contains(JAVA_CHECK_CLASS_FILE_VERSION)
        || log.contains(JAVA_CHECK_CLASS_FILE_VERSION_MMC)
    {
        Some(("‼", RESPONSES.get("use-java-21")?.to_string()))
    } else {
        None
    }
}

fn m1_failed_to_find_service_port(log: &str) -> Option<(&str, String)> {
    const TRIGGER: &str = "java.lang.IllegalStateException: GLFW error before init: [0x10008]Cocoa: Failed to find service port for display";
    if log.contains(TRIGGER) {
        Some((
            "‼",
            RESPONSES
                .get("apple-silicon-incompatible-forge")?
                .to_string(),
        ))
    } else {
        None
    }
}

fn pixel_format_not_accelerated_win10(log: &str) -> Option<(&str, String)> {
    const LWJGL_EXCEPTION: &str = "org.lwjgl.LWJGLException: Pixel format not accelerated";
    const WIN10: &str = "Operating System: Windows 10";
    if log.contains(LWJGL_EXCEPTION) && log.contains(WIN10) {
        Some(("❗", RESPONSES.get("unsupported-intel-gpu")?.to_string()))
    } else {
        None
    }
}

fn intel_graphics_icd_dll(log: &str) -> Option<(&str, String)> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"C  \[(ig[0-9]+icd[0-9]+\.dll)\+(0x[0-9a-f]+)\]").unwrap();
    }
    if RE.is_match(log) {
        Some(("❗", RESPONSES.get("unsupported-intel-gpu")?.to_string()))
    } else {
        None
    }
}

fn java_architecture(log: &str) -> Option<(&str, String)> {
    const TRIGGER: &str = "Your Java architecture is not matching your system architecture.";
    if log.contains(TRIGGER) {
        Some(("❗", RESPONSES.get("32-bit-java")?.to_string()))
    } else {
        None
    }
}

fn detect_temp_directories(log: &str) -> Option<(&str, String)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"Minecraft folder is:\n[A-Z]:/([^/]+/)*Temp").unwrap();
    }
    if log.contains("Rar$") {
        Some(("‼", RESPONSES.get("winrar-temp")?.to_string()))
    }
    else if RE.is_match(log) && !log.contains("forge_installer") {
        Some(("‼", RESPONSES.get("temp-folder")?.to_string()))
    }
    else {
        None
    }
}

fn using_system_openal(log: &str) -> Option<(&str, String)> {
    const TRIGGER: &str = "Using system OpenAL.";
    if log.contains(TRIGGER) {
        Some(("❗", RESPONSES.get("using-system-openal")?.to_string()))
    } else {
        None
    }
}

fn using_system_glfw(log: &str) -> Option<(&str, String)> {
    const TRIGGER: &str = "Using system GLFW.";
    if log.contains(TRIGGER) {
        Some(("❗", RESPONSES.get("using-system-glfw")?.to_string()))
    } else {
        None
    }
}

pub fn common_origins(input: &str) -> Vec<(&str, String)> {
    ORIGINS.iter().flat_map(|m| m(input)).collect()
}

pub(crate) const ORIGINS: [Check; 4] = [
    custom_build,
    pirated_build,
    forked_build,
    m1_wrapper
];

fn custom_build(log: &str) -> Option<(&str, String)> {
    lazy_static! {
        static ref RE_OFFICIAL: Regex = Regex::new(r"MultiMC version: 0\.[0-7]\.[0-9]+-[0-9]+").unwrap();
        static ref RE_CUSTOM: Regex = Regex::new(r"MultiMC version: [a-z0-9\.]+-custom").unwrap();
    }
    if RE_OFFICIAL.is_match(log) {
        None
    } else {
        if RE_CUSTOM.is_match(log) {
            Some(("‼", RESPONSES.get("custom-build")?.to_string()))
        } else {
            None
        }
    }
}

fn pirated_build(log: &str) -> Option<(&str, String)> {
    const PIRATED_BUILD: &str = "UltimMC version: ";
    const AUTH_INJECTOR: &str = "authlib-injector";

    if log.contains(PIRATED_BUILD) || log.contains(AUTH_INJECTOR) {
        Some(("‼", RESPONSES.get("pirated-build")?.to_string()))
    } else {
        None
    }
}

fn forked_build(log: &str) -> Option<(&str, String)> {
    const POLYMC_BUILD: &str = "PolyMC version: ";
    const MANYMC_BUILD: &str = "ManyMC version: ";

    if log.contains(POLYMC_BUILD) || log.contains(MANYMC_BUILD) {
        Some(("‼", RESPONSES.get("forked-build")?.to_string()))
    } else {
        None
    }
}

fn m1_wrapper(log: &str) -> Option<(&str, String)> {
    const M1_PYTHON_WRAPPER: &str = "/m1-multimc-hack/mcwrap.py";

    if log.contains(M1_PYTHON_WRAPPER) {
        Some(("‼", RESPONSES.get("m1-python-wrapper")?.to_string()))
    } else {
        None
    }
}
