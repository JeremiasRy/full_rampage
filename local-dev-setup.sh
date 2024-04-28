#!/bin/bash
echo "Clean up? [y/N]"

read -r response
if [[ "$response" =~ ^([yY]|[yY][eE][sS])$ ]]; then
    cd backend
    echo "Cleaning up target..."
    cargo clean
    echo "target nuked"
    cd ../frontend
    echo "Cleaning up node_modules..."
    rm -rf node_modules
    echo "node_modules nuked"
    if [[ -f "$destination_dir/messages-frontend.proto" ]]; then
        echo "Found existing messages-frontend.proto in $destination_dir. Removing it..."
        rm "$destination_dir/messages-frontend.proto"
    fi
    echo "Clean up done!"
    cd ..
fi

echo "Running this project requires protobuf-compiler installed in your machine. Continue? [Y/n]"
read -r response

if [[ -z "$response" ]]; then 
    response="Y"
fi

if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
    sudo apt-get update && sudo apt install -y protobuf-compiler
fi

proto_bufs="./backend/protos/messages.proto"
destination_dir="./frontend"

cp "$proto_bufs" "$destination_dir"
cd frontend

echo "Installing node_modules"
mv messages.proto messages-frontend.proto
npm i
echo "Installing cargo dependencies"
cd ../backend
cargo build
cd ..

echo "Installed dependinces!"
echo "cd to ./frontend and run 'npm run dev' to run frontend server"
echo "cd to ./backend and run 'cargo run' to run backend server"