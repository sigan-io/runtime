@_default:
    just --list --unsorted

# Formats Justfile
@format:
    just --unstable --fmt

# Installs dependencies for old WP runtime
install:
    composer install --working-dir=old-wp-runtime/prod --optimize-autoloader --no-dev

# Zips old WP runtime
zip:
    mkdir -p dist
    zip dist/runtime.zip old-wp-runtime/prod -rx "*/composer.json" "*/composer.lock" "*/.gitignore" "*/serverless.yml" "*/README.md" "*/CHANGELOG.md" "*/LICENSE" "*/.github/*"

# Installs and zips old WP runtime
bundle:
    just install
    just zip

# Uploads old WP runtime zip to AWS S3
upload:
    aws s3 cp dist/runtime.zip s3://sigan-runtime-code/runtime.zip

# Removes old WP runtime vendor and dist folders
@clean:
    rm -rf old-wp-runtime/prod/vendor
    rm -rf dist

# Builds Docker images and PHP bindings
build target:
    #!/usr/bin/env sh
    if [ "{{ target }}" = "setup" ]; then
      name=sigan/runtime:setup
      docker build --target build-setup --tag $name .

    elif [ "{{ target }}" = "dependencies" ]; then
      name=sigan/runtime:dependencies
      docker build --target build-dependencies --tag $name .

    elif [ "{{ target }}" = "php" ]; then
      name=sigan/runtime:php
      docker build --target build-php --tag $name .

    elif [ "{{ target }}" = "rust" ]; then
      name=sigan/runtime:rust
      docker build --target install-rust --tag $name .

    elif [ "{{ target }}" = "dev" ]; then
      name=sigan/runtime:dev
      docker build --target development --tag $name .

    elif [ "{{ target }}" = "prod" ]; then
      name=sigan/runtime:prod
      docker build --target production --tag $name .

    elif [ "{{ target }}" = "bindings" ]; then
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

# Opens Docker images in interactive mode
open target:
    #!/usr/bin/env sh
    if [ "{{ target }}" = "setup" ]; then
      image=sigan/runtime:setup
      container=sigan-runtime-setup
      docker run \
        --rm \
        --interactive \
        --tty \
        --name $container \
        $image \
        /bin/sh

    elif [ "{{ target }}" = "dev" ]; then
      image=sigan/runtime:dev
      container=sigan-runtime-dev
      docker run \
        --rm \
        --interactive \
        --tty \
        --name $container \
        $image \
        /bin/sh

    elif [ "{{ target }}" = "prod" ]; then
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

# Runs Docker images
run target:
    #!/usr/bin/env sh
    if [ "{{ target }}" = "dev" ]; then
      image=sigan/runtime:dev
      name=sigan-runtime-dev
      HOST_PORT=3000
      docker run \
        --rm \
        --init \
        --env HOST_PORT=$HOST_PORT \
        --publish $HOST_PORT:8080 \
        --volume "$(pwd)/Cargo.toml:/mnt/runtime/Cargo.toml" \
        --volume "$(pwd)/Cargo.lock:/mnt/runtime/Cargo.lock" \
        --volume "$(pwd)/.cargo/config.toml:/mnt/runtime/.cargo/config.toml" \
        --volume "$(pwd)/runtime:/mnt/runtime/runtime" \
        --volume "$(pwd)/php-embed:/mnt/runtime/php-embed" \
        --volume "$(pwd)/php-embed-sys:/mnt/runtime/php-embed-sys" \
        --volume "$(pwd)/wordpress:/mnt/wordpress" \
        --volume "$(pwd)/config:/mnt/config" \
        --volume "$(pwd)/entrypoint.sh:/entrypoint.sh" \
        --entrypoint "/entrypoint.sh" \
        --name $name \
        $image

    else
      echo "Run target unknown.\n"
      exit 1
    fi
