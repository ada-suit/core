#!/usr/bin/env bash

clear;

# copy all source code to the target machine
echo -e "\e[1;33mUpload\n===============\e[1;31m"
scp -r ./src/* core3b+:$HOME/core/src/.

if [ $# -eq 0 ]; then
    # just compile the code
    echo -e "\e[1;33m\nCompile\n===============\e[1;36m"
    ssh core3b+ 'cd ~/core ; cargo build'

elif [ "$1" == "run" ]; then
    # compile and run 
    echo -e "\e[1;33m\nCompile and Run\n===============\e[1;36m"
    ssh core3b+ 'cd ~/core ; cargo run'
fi

