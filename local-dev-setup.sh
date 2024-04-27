#!/bin/bash
echo "Clean up? [y/N]"

read -r response
if [[ "$response" =~ ^([yY]|[yY][eE][sS])$ ]]; then
    echo "Cleaning up target..."
    rm -rf ./backend/target
    echo "target nuked"
    echo "Cleaning up node_modules..."
    rm -rf ./frontend/node_modules
    echo "node_modules nuked"
    if [[ -f "$destination_dir/messages-frontend.proto" ]]; then
        echo "Found existing messages-frontend.proto in $destination_dir. Removing it..."
        rm "$destination_dir/messages-frontend.proto"
    fi
    echo "Clean up done!"
fi

echo "Running this project requires protobuf-compiler installed in your machine. Continue? [Y/n]"
read -r response

if [[ -z "$response" ]]; then 
    response="Y"
fi

if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
    sudo apt-get update && sudo apt install -y protobuf-compiler

    proto_bufs="./backend/protos/messages.proto"
    destination_dir="./frontend"

    cp "$proto_bufs" "$destination_dir"
    cd frontend

    mv messages.proto messages-frontend.proto
    npm i
    cd ../backend
    cargo build
    cd ..
else
    echo "Exiting..."
    exit 0
fi

echo "Installed dependinces!"
echo "cd to ./frontend and run 'npm run dev' to run frontend server"
echo "cd to ./backend and run 'cargo run' to run backend server"