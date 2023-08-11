FROM rust:1.71 AS rust-build

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

COPY . /draught
WORKDIR /draught

RUN wasm-pack build --release
RUN cargo doc --no-deps --document-private-items

FROM node:20-alpine AS js-build

COPY . /draught
WORKDIR /draught

COPY --from=rust-build /draught/pkg /draught/pkg
WORKDIR /draught/www
RUN npm ci
RUN npm run build --if-present
COPY --from=rust-build /draught/target/doc /draught/www/dist/

FROM nginx:alpine-slim
COPY --from=js-build /draught/www/dist /usr/share/nginx/html/