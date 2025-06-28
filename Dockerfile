# Install Bulma with NPM
FROM node:24-alpine AS npm
WORKDIR /app
COPY package.json package-lock.json /app
RUN npm install

# Build app
FROM rust:latest
ENV DEBIAN_FRONTEND=noninteractive
RUN apt update \
    && apt install -y \
    libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libxdo-dev \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*
RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-binstall
RUN cargo binstall dioxus-cli
WORKDIR /app
COPY --from=npm /app/node_modules/bulma/css/bulma.css /app/node_modules/bulma/css/bulma.css
COPY . /app
EXPOSE 8080
ENTRYPOINT ["dx", "serve"]