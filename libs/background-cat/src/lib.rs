#![deny(dead_code)]

pub mod responses;
use responses::RESPONSES;

use lazy_static::lazy_static;
use regex::Regex;

pub fn common_mistakes(input: &str) -> Vec<(&str, String)> {
    PARSERS.iter().flat_map(|m| m(input)).collect()
}

pub(crate) type Check = fn(&str) -> Option<(&str, String)>;

pub(crate) const PARSERS: [Check; 13] = [
    multimc_in_program_files,
    macos_too_new_java,
    multimc_in_onedrive_managed_folder,
    //major_java_version,
    forge_too_new_java,
    one_seventeen_plus_java_too_old,
    m1_failed_to_find_service_port,
    pixel_format_not_accelerated_win10,
    intel_graphics_icd_dll,
    id_range_exceeded,
    out_of_memory_error,
    shadermod_optifine_conflict,
    fabric_api_missing,
    java_architecture,
    //old_multimc_version,
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
/*
fn major_java_version(log: &str) -> Option<(&str, String)> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"Java is version (1.)??(?P<ver>[6-9]|[1-9][0-9])+(\..+)??,").unwrap();
    }
    match RE.captures(log) {
        Some(capture) if capture.name("ver")?.as_str() == "8" => None,
        Some(capture) => Some((
            "❗",
            format!(
                "You're using Java {}. Versions other than Java 8 are not designed to be used with Minecraft and may cause issues. \
                [See here for help installing the correct version.](https://github.com/MultiMC/MultiMC5/wiki/Using-the-right-Java)",
                capture.name("ver")?.as_str()
            ),
        )),
        _ => None,
    }
}
*/

fn forge_too_new_java(log: &str) -> Option<(&str, String)> {
    const URLCLASSLOADER_CAST: &str = "java.lang.ClassCastException: class jdk.internal.loader.ClassLoaders$AppClassLoader cannot be cast to class java.net.URLClassLoader";
    if log.contains(URLCLASSLOADER_CAST) {
        Some(("‼", RESPONSES.get("use-java-8")?.to_string()))
    } else {
        None
    }
}

fn one_seventeen_plus_java_too_old(log: &str) -> Option<(&str, String)> {
    const UNSUPPORTED_CLASS_VERSION_ERROR: &str = "java.lang.UnsupportedClassVersionError: net/minecraft/client/main/Main";
    const FABRIC_JAVA_VERSION_ERROR: &str = "fabric requires {java @ [>=16]}";
    const FABRIC_JAVA_VERSION_ERROR_SEVENTEEN: &str = "fabric requires {java @ [>=17]}";
    const JAVA_16_WARNING: &str = "Minecraft 21w19a and above require the use of Java 16";
    const JAVA_17_WARNING: &str = "Minecraft 1.18 Pre Release 2 and above require the use of Java 17";

    if log.contains(UNSUPPORTED_CLASS_VERSION_ERROR)
        || log.contains(FABRIC_JAVA_VERSION_ERROR)
        || log.contains(FABRIC_JAVA_VERSION_ERROR_SEVENTEEN)
        || log.contains(JAVA_16_WARNING)
        || log.contains(JAVA_17_WARNING)
    {
        Some(("‼", RESPONSES.get("use-java-17")?.to_string()))
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
/* Regex is incorrect/
fn old_multimc_version(log: &str) -> Option<(&str, String)> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"MultiMC version: (?P<major_ver>0\.[0-9]+\.[0-9]+-(?P<build>[0-9]+))\n")
                .unwrap();
    }
    if let Some(capture) = RE.captures(log) {
        match capture.name("build")?.as_str().parse::<u32>() {
            Ok(o) => {
                if o < 900 {
                    Some((
                        "❗",
                        format!(
                            "You seem to be using an old build of MultiMC ({}). \
                            Please update to a more recent version.",
                            capture.name("major_ver")?.as_str()
                        ),
                    ))
                } else {
                    None
                }
            }
            Err(_) => Some((
                "❗",
                format!(
                    "You seem to be using an unofficial version of MultiMC ({}). \
                    Please only use MultiMC downloaded from [multimc.org](https://multimc.org/#Download).",
                    capture.name("major_ver")?.as_str()
                ),
            )),
        }
    } else {
        None
    }
}
*/
