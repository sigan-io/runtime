@default:
  just --list

install:
  composer install --working-dir=old/wp-runtime-prod --optimize-autoloader --no-dev

zip:
  mkdir -p dist
  zip dist/runtime.zip old/wp-runtime-prod -rx "*/composer.json" "*/composer.lock" "*/.gitignore" "*/serverless.yml" "*/README.md" "*/CHANGELOG.md" "*/LICENSE" "*/.github/*"

bundle:
  just install
  just zip

upload:
  aws s3 cp dist/runtime.zip s3://sigan-runtime-code/runtime.zip

clean:
  rm -rf wp-runtime-prod/vendor
  rm -rf dist

build-libraries:
  docker build --target build-libraries --tag libraries .

build-build-php:
  docker build --target build-php --tag build-php .

build-bindings-image:
  docker build --target build-bindings --tag build-bindings .

build-php:
  docker build --target php --tag php .

build-dev:
  docker build --target runtime-dev --tag runtime-dev .

run-libraries:
  docker run --rm --name libraries libraries /bin/sh

run-build-php:
  docker run --rm --name build-php build-php /bin/sh

run-php:
  docker run --rm --name php php /bin/sh

run-dev:
  export HOST_PORT=3000 && \
  docker run \
    --rm \
    --init \
    --env HOST_PORT=$HOST_PORT \
    --publish $HOST_PORT:8080 \
    --volume "$(pwd)/Cargo.toml:/mnt/runtime/Cargo.toml" \
    --volume "$(pwd)/Cargo.lock:/mnt/runtime/Cargo.lock" \
    --volume "$(pwd)/runtime:/mnt/runtime/runtime" \
    --volume "$(pwd)/php-lambda:/mnt/runtime/php-lambda" \
    --volume "$(pwd)/fastcgi:/mnt/runtime/fastcgi" \
    --volume "$(pwd)/old-fastcgi-client:/mnt/runtime/old-fastcgi-client" \
    --volume "$(pwd)/litespeed-client:/mnt/runtime/litespeed-client" \
    --volume "$(pwd)/php-embed-sys:/mnt/runtime/php-embed-sys" \
    --volume "$(pwd)/wordpress:/mnt/wordpress" \
    --volume "$(pwd)/config:/mnt/config" \
    --name runtime-dev \
    runtime-dev

bindings command="run":
  docker run \
    --rm \
    --volume "$(pwd)/php-embed-sys:/mnt/runtime/php-embed-sys" \
    --name build-bindings \
    build-bindings \
    {{command}}

# Builds Docker images or Rust libraries depending on the target.
build target:
  #!/usr/bin/env sh
  if [ "{{target}}" = "setup" ]; then
    name=sigan/runtime:setup
    docker build --target build-setup --tag $name .
    docker image rm $name

  elif [ "{{target}}" = "dependencies" ]; then
    name=sigan/runtime:dependencies
    docker build --target build-dependencies --tag $name .
    docker image rm $name

  elif [ "{{target}}" = "php" ]; then
    name=sigan/runtime:php
    docker build --target build-php --tag $name .
    docker image rm $name

  elif [ "{{target}}" = "dev" ]; then
    name=sigan/runtime:dev
    docker build --target development --tag $name .
  
  elif [ "{{target}}" = "prod" ]; then
    name=sigan/runtime:prod
    docker build --target production --tag $name .

  elif [ "{{target}}" = "bindings" ]; then
    image=sigan/runtime:dev
    container=sigan-runtime-dev
    docker run \
      --rm \
      --volume "$(pwd)/php-embed-sys:/mnt/runtime/php-embed-sys" \
      --env "RUNNING_IN_DOCKER=true" \
      --name $container \
      $image \
      /bin/sh -c "cd php-embed-sys && cargo build"

  else
    echo "Build target unknown.\n"
    exit 1
  fi

open target:
  #!/usr/bin/env sh
  if [ "{{target}}" = "dev" ]; then
    image=sigan/runtime:dev
    container=sigan-runtime-dev
    docker run \
      --rm \
      --interactive \
      --tty \
      --name $container \
      $image \
      /bin/sh

  elif [ "{{target}}" = "prod" ]; then
    image=sigan/runtime:prod
    container=sigan-runtime-prod
    docker run \
      --rm \
      --interactive \
      --tty \
      --name $container \
      $image \
      /bin/sh

  else
    echo "Open target unknown.\n"
    exit 1
  fi

# @run target:
#   if [ "{{target}}" = "setup"] then
#     docker run --rm --name setup setup
