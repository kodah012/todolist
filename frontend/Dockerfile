FROM node:18 AS builder
WORKDIR /frontend
COPY . .
RUN npm install
RUN npm run build

FROM nginx:alpine
WORKDIR /dist
COPY --from=builder /frontend/dist .
COPY ./nginx.conf /etc/nginx/nginx.conf
