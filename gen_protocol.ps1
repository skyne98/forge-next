mkdir -Force lib/protocol/
protoc --dart_out=lib ./protocol/client.proto
protoc --dart_out=lib ./protocol/server.proto