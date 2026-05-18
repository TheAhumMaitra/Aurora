# Aurora's Cli

Aurora's cli is automatically insatlled when you use the installtion script or if you followed manual installation script correctly. You can do many things with Aurora's cli. 

## Some useful commands
`aurora --help` to view all available commands and flags.

```bash
aurora apply-theme "THEME_NAME"
```
This command can apply your instructed theme

```bash 
aurora list-themes
```
This will show all available themes

## All sub-commands
| Sub-command | Action |
| :-- | :-- |
| `version` | `Shows current Aurora's version and Aurora's cli version`|
| `apply-theme [ARG_THEME_NAME]` | `Applies given theme globally` |
| ` list-themes` | `Lists all available themes` |
| `information` | `Shows information about Aurora` |
| `theme-info` | `Shows information about current theme` |
| `download-theme [ARG_GIT_URL]` | `Downloads the theme` | 
| `refresh` | `Refresh system` |
| `help` | `Shows help` |