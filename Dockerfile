FROM rust:1.69 AS rust-build

RUN  curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

COPY . ./

RUN wasm-pack build --release
RUN cargo doc --no-deps --document-private-items

FROM node:18 AS js-build

COPY . ./
COPY --from=rust-build /pkg /pkg
WORKDIR /www
RUN npm ci
RUN npm run build --if-present
COPY --from=rust-build /target/doc /www/dist/

FROM nginx
COPY --from=js-build /www/dist /usr/share/nginx/html/