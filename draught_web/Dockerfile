FROM rust:1.71 AS rust-build

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

COPY .. /draught
WORKDIR /draught/draught_web

RUN wasm-pack build --release

WORKDIR /draught
RUN cargo doc --no-deps --document-private-items

FROM node:20-alpine AS js-build

COPY .. /draught
WORKDIR /draught/draught_web

COPY --from=rust-build /draught/draught_web/pkg /draught/draught_web/pkg
WORKDIR /draught/draught_web/www
RUN npm ci
RUN npm run build --if-present
COPY --from=rust-build /draught/target/doc /draught/draught_web/www/dist/

FROM nginx:alpine-slim
COPY --from=js-build /draught/draught_web/www/dist /usr/share/nginx/html/