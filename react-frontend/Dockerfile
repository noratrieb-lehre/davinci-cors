FROM node:18 as build
WORKDIR /app
ENV PATH /app/node_modules/.bin:$PATH
COPY package.json ./
RUN NODE_OPTIONS=--openssl-legacy-provider npm install
COPY package.json yarn.lock tsconfig.json src public ./
RUN NODE_OPTIONS=--openssl-legacy-provider npm run build

FROM nginx:stable-alpine
COPY ./nginx/nginx.conf /etc/nginx/templates/default.conf.template
COPY --from=build /app/build /usr/share/nginx/html
CMD ["nginx", "-g", "daemon off;"]
