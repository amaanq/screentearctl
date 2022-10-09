use clap::Parser;

/// A utility to control screen tearing on Linux (X11 only!)
///
/// The main use case is to disable screen tearing for a specific application where lower latency
/// might be key (for example, a game)
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct ScreenTearArgs {
    // /// Print output in format FMT
    // #[clap(long, short, default_value = "text")]
    // pub output: OutFormat,
    /// The comic to load
    // #[clap(long, short)]
    // pub num: Option<usize>,
    // /// Save image file to current directory
    // #[clap(long, short)]
    // pub save: bool,

    /// The display to change the settings for
    #[clap(long, short, required = true, value_name = "DISPLAY_NAME", hide_possible_values = true, value_parser = get_screens)]
    pub display: Option<String>,
}

/// Gets the display names from the output of `xrandr`
pub fn get_screens(partial: &str) -> Result<Vec<String>, String> {
    let xrandr_output = std::process::Command::new("xrandr")
        .arg("2>/dev/null")
        .output();

    // .expect("failed to execute `xrandr`, are you on X11 and is `xrandr` installed?");
    /*  let xrandr_output = String::from_utf8(xrandr_output.stdout).unwrap(); */

    let xrandr_output = match xrandr_output {
        Ok(output) => String::from_utf8(output.stdout).unwrap(),
        Err(_) => {
            return Err(String::from(
                "failed to execute `xrandr`, are you on X11 and is `xrandr` installed?",
            ))
        }
    };

    let regex_matcher = regex::Regex::new(r"^([a-zA-Z0-9\-]+) connected(.*)").unwrap();

    let mut screens = Vec::new();
    for line in xrandr_output.lines() {
        if regex_matcher.is_match(line) {
            screens.push(regex_matcher.replace(line, "$1").to_string());
        }
    }

    screens.retain(|screen| screen.contains(partial));
    Ok(screens)
}
