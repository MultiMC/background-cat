use indoc::indoc;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref RESPONSES: HashMap<&'static str, &'static str> = HashMap::from([
        (
            "program-files",
            indoc! {"
                Your MultiMC installation is in Program Files, where MultiMC doesn't have permission to write.\n\
                You should move it somewhere else, like your Desktop."
            }
        ),
        (
            "macos-java-too-new",
            indoc! {
            "You are using too new a Java version. Please follow the steps on this wiki page to install 8u241: https://github.com/MultiMC/MultiMC5/wiki/Java-on-macOS"
            }
        ),
        (
            "id-limit",
            indoc! {
                    "You've exceeded the hardcoded ID Limit. Remove some mods, or install [JustEnoughIDs](https://www.curseforge.com/minecraft/mc-mods/jeid)"
            }
        ),
        (
            "out-of-memory",
            indoc! {
                "You've run out of memory. You should allocate more, although the exact value depends on how many mods you have installed. \
                [Click this link for a guide.](https://cdn.discordapp.com/attachments/531598137790562305/575376840173027330/unknown.png)"
            }
        ),
        (
            "optifine-and-shadermod",
            indoc! {
                "You've installed Shaders Mod alongside OptiFine. OptiFine has built-in shader support, so you should remove Shaders Mod"
            }
        ),
        (
            "missing-fabric-api",
            indoc! {
                "You are missing Fabric API, which is required by a mod. \
                [Download the needed version here](https://www.curseforge.com/minecraft/mc-mods/fabric-api)"
            }
        ),
        (
            "multimc-in-onedrive",
            indoc! {
                "MultiMC is possibly located in a folder managed by OneDrive, if you already have OneDrive disabled \
                you can ignore this warning.\n\
                OneDrive can mess with your game files while the game is running, and this often leads to crashes.\n\
                You should move the MultiMC folder to a different folder."
            }
        ),
        (
            "use-java-8",
            indoc! {
                "The version of Minecraft you are playing does not support using modern versions of Java.\n\
                [Please check our wiki for more information.](https://github.com/MultiMC/Launcher/wiki/Using-the-right-Java#minecraft-116-and-older)"
            }
        ),
        (
            "use-java-17",
            indoc! {
                "You are playing a version of Minecraft that requires Java 17 or newer, but are using an older Java version.\n\
                [Please check our wiki for more information.](https://github.com/MultiMC/Launcher/wiki/Using-the-right-Java#minecraft-117-and-newer)"
            }
        ),
        (
            "apple-silicon-incompatible-forge",
            indoc! {
                "You seem to be using an Apple M1 Mac with an incompatible version of Forge. Add the following to your launch arguments as a workaround: `-Dfml.earlyprogresswindow=false`"
            }
        ),
        (
            "unsupported-intel-gpu",
            indoc! {
                "You seem to be using an Intel GPU that is not supported on Windows 10. \
                You will need to install an older version of Java, [see here for help](https://github.com/MultiMC/MultiMC5/wiki/Unsupported-Intel-GPUs)"
            }
        ),
        (
            "32-bit-java",
            indoc! {
                "You're using 32-bit Java. [See here for help installing the correct version.](https://github.com/MultiMC/MultiMC5/wiki/Using-the-right-Java)"
            }
        ),
        (
            "custom-build",
            indoc! {
                "You're using a self build version of MultiMC. We only support builds distributed from our website or packages from our wiki. [See here for official builds.](https://multimc.org)\n\
                If you are building MultiMC as developer to add features, you might want to request access to #dev-talk to discuss the feature."
            }
        ),
        (
            "pirated-build",
            indoc! {
                "You're using a pirated Launcher. We only support builds distributed from our website or packages from our wiki that only work with a Mojang and Microsoft Account. [See here for official builds.](https://multimc.org)"
            }
        ),
        (
            "forked-build",
            indoc! {
                "You're using a forked build of MultiMC. We only support builds distributed from our website or packages from our wiki. [See here for official builds.](https://multimc.org)"
            }
        ),
        (
            "m1-python-wrapper",
            indoc! {
                "You're using MultiMC with the M1 python wrapper. Please do not use this to make MultiMC work on M1. Instead run MultiMC via the Rosetta 2 compatibility layer and with the usage of a x86_64 Java binary."
            }
        ),
        (
            "using-system-openal",
            indoc! {
                "You seem to be using your systems OpenAL installation. This can cause the instance to crash if not properly setup. In case of a crash, make sure this isn't the cause of it."
            }
        ),
        (
            "using-system-glfw",
            indoc! {
                "You seem to be using your systems GLFW installation. This can cause the instance to crash if not properly setup. In case of a crash, make sure this isn't the cause of it."
            }
        ),
        (
            "reboot-required",
            indoc! {
                "Another process appears to be locking your native library JARs. To solve this, please reboot your computer."
            }
        )
    ]);
}
