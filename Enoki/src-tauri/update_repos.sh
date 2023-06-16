
# use this script to update all the repos

#if cwd does not end with /src-tauri/ then exit
if [[ $PWD != */src-tauri ]]; then
    echo "Please run this script from the src-tauri directory"
    exit 1
fi


cd ..

# check if wpilog-rs dir exists
if [ ! -d "wpilog-rs" ]; then
    git clone https://github.com/oh-yes-0-fps/wpilog-rs.git
fi

cd ./wpilog-rs/
git pull

cd ..

# check if network-tables-rs dir exists
if [ ! -d "network-tables-rs" ]; then
    git clone https://github.com/oh-yes-0-fps/network-tables-rs.git
fi

cd ./network-tables-rs/
git pull
