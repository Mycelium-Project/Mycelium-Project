# to run this you need to run powershell as admin and rn this command
# Set-ExecutionPolicy -ExecutionPolicy RemoteSigned
# then you can run this script by typing .\update_repos.ps1

# use this script to update all the repos

# if cwd does not end with src-tauri then exit
if ($PWD.Path.EndsWith("src-tauri")) {
    echo "cwd is src-tauri"
} else {
    echo "cwd is not src-tauri"
    exit
}

cd ..

# check if wpilog-rs dir exists for a powershell script
if (Test-Path ./wpilog-rs) {
    echo "wpilog-rs exists"
} else {
    echo "wpilog-rs does not exist"
    git clone https://github.com/oh-yes-0-fps/wpilog-rs.git
}


cd ./wpilog-rs/
git pull

cd ..

# check if network-tables-rs dir exists for a powershell script
if (Test-Path ./network-tables-rs) {
    echo "network-tables-rs exists"
} else {
    echo "network-tables-rs does not exist"
    git clone https://github.com/oh-yes-0-fps/network-tables-rs.git
}

cd ./network-tables-rs/
git pull

cd ../src-tauri/