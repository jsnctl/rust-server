docker build -t rust-server .
docker run --rm -p 8081:8081 --init rust-server