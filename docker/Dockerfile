# Rust'ın en güncel sürümünü kullanın
FROM rust:1.75

# Set the application directory
WORKDIR /app

# Gerekli bağımlılıkları yükleyin
RUN apt-get update && apt-get install -y musl-tools && apt-get clean

# cargo-watch kurulumunu yapın
RUN cargo install cargo-watch

# Uygulama dosyalarını kopyalayın
COPY . .

CMD ["cargo", "watch", "-x", "run"]


# FROM rust:latest

# WORKDIR /usr/src/app

# COPY Cargo.toml Cargo.lock ./
# RUN cargo build --release
# COPY src ./src

# RUN cargo install cargo-watch

# CMD ["cargo", "watch", "-x", "run"]
