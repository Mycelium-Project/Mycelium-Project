# to run this you need to run powershell as admin and rn this command
# Set-ExecutionPolicy -ExecutionPolicy RemoteSigned
# then you can run this script by typing .\update_repos.ps1

# use this script to update all the repos
cd ../wpilog-rs/
git pull

cd ../network-tables-rs/
git pull

cd ../src-tauri/
git pull