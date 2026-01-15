# launchy
Web App launcher inspired by Omarchy

✨ Motivation

I really love how Omarchy handles web applications:
they feel like native apps, easy to launch, and unobtrusive.

However, I didn’t want to keep adding more and more keybindings to my system just to open different web tools (mail, dashboards, docs, chat, etc). Over time this becomes hard to maintain, remember and conflicts easily.

This project exists to solve that:
- 🚀 Fast access to all your web apps from a single launcher
- 🧹 No keybinding pollution
- 🧠 Simple mental model: one shortcut → everything
- 🪶 Lightweight and minimal by design

🧩 What It Does

- Launches web applications from a centralized interface
- Each web app behaves like its own isolated app/window
- Keeps your global keybindings clean
- Designed to integrate nicely with hyprland
- Easily extensible configuration

🛠️ Philosophy

- Minimalism over features
- Keyboard-first UX
- Predictable behavior
- Hackable and transparent configuration
- No heavy dependencies unless absolutely necessary

🚧 Project Status

This project is currently under active development.
Expect breaking changes, rough edges, and rapid iteration.

📦 Compilation -> cargo build

▶️ Usage

cargo run

If you like it I suggest building with "cargo build --release" then copying the produced binary (target/release/launchy) into $HOME/.local/bin/ and adding a couple of rules to hyprland:

- bindd = SUPER SHIFT, L, Launchy, exec, ~/.local/bin/launchy
- windowrule = match:title ^Launchy$, float on

Inside the app you can use Ctrl/Alt+Number to launch a favorite or type an url.

⚙️ Configuration

Everything is configured with a TOML file that's expected to live inside $HOME/.config/launchy/conf.toml
The app will also create an icon cache in $HOME/.cache/launchy/

The config file is a list of services, for example:

```toml
services = [
    { name = "Gmail", url = "https://gmail.com", icon_url = "https://ssl.gstatic.com/ui/v1/icons/mail/rfr/gmail.ico" },
    { name = "GitHub", url = "https://github.com", icon_url = "https://github.com/favicon.ico" },
    { name = "RustDocs", url = "https://docs.rs", icon_url = "https://docs.rs/favicon.ico" }
]
```

🤝 Contributing

Pull requests are welcome.
If you have ideas, improvements, or bug reports, feel free to open an issue or discussion.
