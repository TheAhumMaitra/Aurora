export type ThemeStatus = "official" | "external";

export type Theme = {
  id: string;
  name: string;
  repoUrl: string;
  status: ThemeStatus;
  previewImageUrl: string;
};

export const themes: Theme[] = [
  {
    id: "0001",
    name: "Aurora Default",
    repoUrl: "https://github.com/TheAhumMaitra/Aurora/tree/hyprland-git-lua/dotfiles/.config/themes/Aurora%20Default",
    status: "official",
    previewImageUrl:
      "https://raw.githubusercontent.com/TheAhumMaitra/Aurora/refs/heads/hyprland-git-lua/dotfiles/.config/themes/Aurora%20Default/preview.png",
  },
  {
    id: "0002",
    name: "Aurora Glassy",
    repoUrl: "https://github.com/TheAhumMaitra/Aurora/tree/hyprland-git-lua/dotfiles/.config/themes/Aurora%20glassy",
    status: "official",
    previewImageUrl:
      "https://raw.githubusercontent.com/TheAhumMaitra/Aurora/refs/heads/hyprland-git-lua/dotfiles/.config/themes/Aurora%20glassy/preview.png",
  },
  {
    id: "0003",
    name: "Catppuccin Mocha",
    repoUrl: "https://github.com/TheAhumMaitra/Aurora/tree/hyprland-git-lua/dotfiles/.config/themes/Catppuccin%20Mocha",
    status: "official",
    previewImageUrl:
      "https://raw.githubusercontent.com/TheAhumMaitra/Aurora/refs/heads/hyprland-git-lua/dotfiles/.config/themes/Catppuccin%20Mocha/preview.png",
  },
  {
    id: "0004",
    name: "Dracula",
    repoUrl: "https://github.com/TheAhumMaitra/Aurora/tree/hyprland-git-lua/dotfiles/.config/themes/Dracula",
    status: "official",
    previewImageUrl:
      "https://raw.githubusercontent.com/TheAhumMaitra/Aurora/refs/heads/hyprland-git-lua/dotfiles/.config/themes/Dracula/preview.png",
  },
  {
    id: "0005",
    name: "Everforest",
    repoUrl: "https://github.com/TheAhumMaitra/Aurora/tree/hyprland-git-lua/dotfiles/.config/themes/Everforest",
    status: "official",
    previewImageUrl:
      "https://raw.githubusercontent.com/TheAhumMaitra/Aurora/refs/heads/hyprland-git-lua/dotfiles/.config/themes/Everforest/preview.png",
  },
  {
    id: "0006",
    name: "Gruvbox",
    repoUrl: "https://github.com/TheAhumMaitra/Aurora/tree/hyprland-git-lua/dotfiles/.config/themes/Gruvbox",
    status: "official",
    previewImageUrl:
      "https://raw.githubusercontent.com/TheAhumMaitra/Aurora/refs/heads/hyprland-git-lua/dotfiles/.config/themes/Gruvbox/preview.png",
  },
  {
    id: "0007",
    name: "Nord",
    repoUrl: "https://github.com/TheAhumMaitra/Aurora/tree/hyprland-git-lua/dotfiles/.config/themes/Nord",
    status: "official",
    previewImageUrl:
      "https://raw.githubusercontent.com/TheAhumMaitra/Aurora/refs/heads/hyprland-git-lua/dotfiles/.config/themes/Nord/preview.png",
  },
  {
    id: "0008",
    name: "Rose Pine",
    repoUrl: "https://github.com/TheAhumMaitra/Aurora/tree/hyprland-git-lua/dotfiles/.config/themes/Rose%20Pine",
    status: "official",
    previewImageUrl:
      "https://raw.githubusercontent.com/TheAhumMaitra/Aurora/refs/heads/hyprland-git-lua/dotfiles/.config/themes/Rose%20Pine/preview.png",
  },
  {
    id: "0009",
    name: "Tokyo Night",
    repoUrl: "https://github.com/TheAhumMaitra/Aurora/tree/hyprland-git-lua/dotfiles/.config/themes/Tokyo%20Night",
    status: "official",
    previewImageUrl:
      "https://raw.githubusercontent.com/TheAhumMaitra/Aurora/refs/heads/hyprland-git-lua/dotfiles/.config/themes/Tokyo%20Night/preview.png",
  },
  {
    id: "0010",
    name: "Osaka Jade",
    repoUrl: "https://github.com/TheAhumMaitra/Osaka-Jade-For-Aurora",
    status: "external",
    previewImageUrl:
     "https://raw.githubusercontent.com/TheAhumMaitra/Osaka-Jade-For-Aurora/refs/heads/master/preview.png",
  },
];

export function getThemeSlug(name: string) {
  return name.toLowerCase().trim().replaceAll(/[^a-z0-9]+/g, "-");
}

export function getThemeBySlug(slug: string) {
  return themes.find((theme) => getThemeSlug(theme.name) === slug);
}
