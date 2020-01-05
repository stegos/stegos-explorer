FROM node:8.15.0-alpine as build-stage

ENV YARN_CACHE_FOLDER /yarn
WORKDIR /app/

# Add entrypoint script to handle yarn cache
RUN mkdir -p $YARN_CACHE_FOLDER
COPY package.json yarn.lock ./

# install dependencies befure copy source, to prevent fetching on each build.
RUN yarn install --frozen-lockfile

ARG SERVICE_IP
ENV SERVICE_IP=$SERVICE_IP
COPY . .
RUN yarn build

# Produce final image optimized for size.
FROM nginx:stable-alpine as production-stage
COPY --from=build-stage /app/dist /usr/share/nginx/html
# keep /etc/nginx/nginx.conf with include statement for all /etc/nginx/conf.d/*.conf
# but remove /etc/nginx/conf.d/default.conf for approbriate sub-path settings
RUN rm /etc/nginx/conf.d/default.conf
COPY app.conf /etc/nginx/conf.d/app.conf

EXPOSE 80

WORKDIR /etc/nginx
CMD ["nginx", "-g", "daemon off;"]