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
    --volume "$(pwd)/php-sys:/mnt/runtime/php-sys" \
    --volume "$(pwd)/wordpress:/mnt/wordpress" \
    --volume "$(pwd)/config:/mnt/config" \
    --name runtime-dev \
    runtime-dev

@bindings command="run":
  docker run \
    --rm \
    --volume "$(pwd)/php-sys:/mnt/runtime/php-sys" \
    --name build-bindings \
    build-bindings \
    {{command}}