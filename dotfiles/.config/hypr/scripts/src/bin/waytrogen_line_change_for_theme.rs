use std::fs;

fn main() -> std::io::Result<()> {
    let home = std::env::var("HOME").unwrap();

    let path = format!("{}/.config/waytrogen/config.json", home);

    let content = fs::read_to_string(&path)?;

    let theme_name = fs::read_to_string(format!("{}/.local/share/Aurora/theme_name.log", home))?;

    let theme_name = theme_name.trim();

    let wallpapers = format!("{}/.config/themes/{}/backgrounds", home, theme_name);

    let mut lines: Vec<String> = content.lines().map(String::from).collect();

    // Find wallpaper_folder line automatically
    for line in &mut lines {
        if line.contains("\"wallpaper_folder\"") {
            *line = format!("  \"wallpaper_folder\": \"{}\",", wallpapers);
        }
    }

    let new_content = lines.join("\n");

    fs::write(path, new_content)?;

    println!("Updated!");

    Ok(())
}
