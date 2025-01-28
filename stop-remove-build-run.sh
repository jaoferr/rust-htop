docker stop rust-htop \
& docker rm rust-htop \
& docker build -t rust-htop . \
&& docker run --rm -p 7032:7032 --name=rust-htop rust-htop