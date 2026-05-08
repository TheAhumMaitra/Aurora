use std::fs;

fn main() -> std::io::Result<()> {
    let home = std::env::var("HOME").unwrap();

    let path =
        format!("{}/.config/waytrogen/config.json", home);

    let content = fs::read_to_string(&path)?;

    let mut lines: Vec<String> =
        content.lines().map(String::from).collect();

    // Find wallpaper_folder line automatically
    for line in &mut lines {
        if line.contains("\"wallpaper_folder\"") {
            *line = format!(
                "  \"wallpaper_folder\": \"~/Pictures/Wallpapers/\",",
            );
        }
    }

    let new_content = lines.join("\n");

    fs::write(path, new_content)?;

    println!("Updated!");

    Ok(())
}