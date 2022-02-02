cd ~/Desktop/Rust
git clone https://github.com/eboatwright/rutoe_template
mv rutoe_template $1
cd $1
rm -rf .git
rm .gitattributes README.md .gitignore
sed -i "s/rutoe_template/$1/g" Cargo.toml
sed -i "s/CRATE_NAME/$1/g" web/index.html
sed -i "s/CRATE_NAME/$1/g" web/build_wasm.sh