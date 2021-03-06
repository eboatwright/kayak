# Notice
 This is extremely early in development, and major changes are happening all the time!

# Kayak
 Kayak is a simple to use, non ECS "Game Engine" for Rust and Macroquad!<br>
 Supports Windows, Mac, Linux and Web Assembly!<br><br>

# Creating a New Project
 To create a new project with Kayak, you can do this command:
```
git clone https://github.com/eboatwright/kayak_template
```
 Or, go to the template repository, and click: "Use this Template"!

# Running a Kayak Project
 You can run a project by going into your project's root directory and running the command:
```
./run.sh
```
 or
```
sh run.sh
```
 (On Linux and Mac you might need to do this command before):
```
chmod +x run.sh
```
 Note: The build scripts use --release by default so that's why you might have slow build times.

# Distributing
 If your game uses textures, sounds or fonts (besides defaults) copy the res folder in src and ship it with your executable.

# WASM
 Go into the web folder, and replace all CRATE_NAME in index.html and build_wasm.sh with your crate name, then run the shell file!<br>
 You can use a program like basic-http-server in the web folder for testing.

# This Project Uses My Modified Version of Macroquad
 Check out the original here!<br>
 https://github.com/not-fl3/macroquad

# License & Copyright
 This repository is under the MIT license.<br>
 Check the LICENSE file for more information.
