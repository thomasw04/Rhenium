# RustyBear-Engine  
![Verify and Tests](https://github.com/thomasw04/RustyBear-Engine/actions/workflows/verify.yml/badge.svg)

Game-Engine experiment written in Rust.

The plan is to provide Vulkan support for Windows + Linux and native Metal support for macOS.  

At the beginning, I will try to make rendering a triangle work. Then I will decide where to go with this project.

## Install

1. Make sure you have rustup, cmake, ninja and the VulkanSDK installed.

2. Clone the repository and run ```cargo run``` in the project folder.

3. It should work on all desktop platforms that support Vulkan (or Metal when its implemented). For debugging in vsc you will need the CodeLLDB extension.
