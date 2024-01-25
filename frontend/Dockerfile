FROM node:20 as builder

WORKDIR /frontend

COPY ./ ./

RUN npm install -g pnpm

RUN pnpm install

RUN pnpm run build

CMD echo "success"
