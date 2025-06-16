FROM rust:1.87-slim

RUN apt-get update && apt-get install -y \
    curl \
    pkg-config \
    libpq-dev

WORKDIR /app
COPY . .

CMD ["tail", "-f", "/dev/null"]