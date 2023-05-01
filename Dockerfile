FROM rust:1.69 AS rust-build

RUN  curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

COPY . /draught
WORKDIR /draught

RUN wasm-pack build --release
RUN ls
RUN cargo doc --no-deps --document-private-items

FROM node:18 AS js-build

COPY . /draught
WORKDIR /draught

COPY --from=rust-build /draught/pkg /draught/pkg
WORKDIR /draught/www
RUN npm ci
RUN npm run build --if-present
COPY --from=rust-build /draught/target/doc /draught/www/dist/

FROM nginx
COPY --from=js-build /draught/www/dist /usr/share/nginx/html/