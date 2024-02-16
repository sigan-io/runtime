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
  docker run \
    --rm \
    --init \
    --publish 3000:8080 \
    --volume ./Cargo.toml:/mnt/runtime/Cargo.toml \
    --volume ./Cargo.lock:/mnt/runtime/Cargo.lock \
    --volume ./src:/mnt/runtime/src \
    --volume ./target:/mnt/runtime/target \
    --name runtime-dev \
    runtime-dev