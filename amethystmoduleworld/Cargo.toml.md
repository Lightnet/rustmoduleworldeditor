[package]
name = "amethystmoduleworld"
version = "0.1.0"
authors = ["Lightnet <>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

#[[bin]]
#name = "amethystmoduleworld"
#path = "src/main.rs"
#path = "src/mainimgui.rs"

[[bin]]
name = "amethystserver"
path = "src/server.rs"

[[bin]]
name = "amethystclient"
path = "src/client.rs"

[dependencies]
##amethyst-imgui = { version="0.3.1", features = ["vulkan"] }
#amethyst-imgui = { version="0.3.1" }
#amethyst-imgui = { path = "../amethyst-imgui", features = ["vulkan"] }
amethyst-imgui = "0.3.1"

# cargo run -p amethystmoduleworld
#[dependencies.amethyst-imgui]
#version = "0.3.1"
# #features = ["vulkan"]

#https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-01.html
[dependencies.amethyst]
version = "0.12.0"
#default-features = false
features = ["vulkan"]
