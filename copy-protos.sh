proto_bufs="./backend/protos/messages.proto"
destination_dir="./frontend/build"

cp "$proto_bufs" "$destination_dir"
cd frontend

mv messages.proto messages-frontend.proto