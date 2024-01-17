#!/usr/bin/env bash

# copy all source code to the target machine
echo -e "\e[1;33mUpload\n===============\e[1;31m"
scp -r ./src/* core3b+:$HOME/ada/src/.

if [ $# -eq 0 ]; then
    # just compile the code
    echo -e "\e[1;33m\nCompile\n===============\e[1;36m"
    ssh core3b+ 'cd ~/ada ; cargo build'

elif [ "$1" == "run" ]; then
    # compile and run 
    echo -e "\e[1;33m\nCompile and Run\n===============\e[1;36m"
    ssh core3b+ 'cd ~/ada ; cargo run'
fi
