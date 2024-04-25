# syntax=docker/dockerfile:1

###############################
### Setup Build Environment ###
###############################

FROM public.ecr.aws/lambda/provided:al2023-arm64 as build-setup

ENTRYPOINT []

ARG BUILD_DIR=/tmp/build
ARG INSTALL_DIR=/opt

# Set PATH environment variable to include installed binaries
ARG PATH=${INSTALL_DIR}/bin:${INSTALL_DIR}/sbin:${PATH}

# Set LD_LIBRARY_PATH to include installed libraries
ARG LD_LIBRARY_PATH=${INSTALL_DIR}/lib:${INSTALL_DIR}/lib64:${LD_LIBRARY_PATH}

# Set the PKG_CONFIG_PATH to include libraries built from source
ARG PKG_CONFIG_PATH=${INSTALL_DIR}/lib64/pkgconfig:${INSTALL_DIR}/lib/pkgconfig

# Install utilities needed to build PHP and its extensions

RUN LD_LIBRARY_PATH= dnf install -y gzip tar xz gcc g++ re2c bison cmake autoconf automake libtool binutils perl glibc-locale-source

# Locale settings

RUN localedef -i en_US -f UTF-8 en_US.UTF-8

ARG LANGUAGE=en_US.UTF-8
ARG LC_ALL=en_US.UTF-8
ARG LANG=en_US.UTF-8

##############################
### Build PHP Dependencies ###
##############################

FROM build-setup as build-dependencies

# Build Zlib
# Needed by:
#   - openssl
#   - php-zlib
#   - libzip

WORKDIR ${BUILD_DIR}/zlib/

RUN set -e \
  && ZLIB_VERSION=1.3.1 \
  && curl --location --silent --show-error --fail https://github.com/madler/zlib/releases/download/v${ZLIB_VERSION}/zlib-${ZLIB_VERSION}.tar.gz \
  | tar xzvC . --strip-components=1

RUN CFLAGS="-O3" \
  CPPFLAGS="-I${INSTALL_DIR}/include  -I/usr/include" \
  LDFLAGS="-L${INSTALL_DIR}/lib64 -L${INSTALL_DIR}/lib" \
  ./configure \
  --prefix=${INSTALL_DIR}

RUN make -j$(nproc) \
  && make install \
  && make clean \
  && rm -rf ${BUILD_DIR}/zlib/

# Build ICU
# Needed by:
#   - php-intl

WORKDIR ${BUILD_DIR}/icu/

RUN set -e \
  && ICU_VERSION=74.2 \
  && curl --location --silent --show-error --fail https://github.com/unicode-org/icu/releases/download/release-${ICU_VERSION//./-}/icu4c-${ICU_VERSION//./_}-src.tgz \
  | tar xzC . --strip-components=1

WORKDIR ${BUILD_DIR}/icu/source/

RUN CFLAGS="-O3" \
  CPPFLAGS="-I${INSTALL_DIR}/include  -I/usr/include" \
  LDFLAGS="-L${INSTALL_DIR}/lib64 -L${INSTALL_DIR}/lib" \
  ./configure \
  --prefix=${INSTALL_DIR}

RUN make -j$(nproc) \
  && make install \
  && make clean \
  && rm -rf ${BUILD_DIR}/icu/

# Build Oniguruma
# Needed by:
#   - php-mbstring

WORKDIR ${BUILD_DIR}/oniguruma/

RUN set -e \
  && ONIGURUMA_VERSION=6.9.9 \
  && curl --location --silent --show-error --fail https://github.com/kkos/oniguruma/releases/download/v${ONIGURUMA_VERSION}/onig-${ONIGURUMA_VERSION}.tar.gz \
  | tar xzC . --strip-components=1

RUN CFLAGS="-O3" \
  CPPFLAGS="-I${INSTALL_DIR}/include  -I/usr/include" \
  LDFLAGS="-L${INSTALL_DIR}/lib64 -L${INSTALL_DIR}/lib" \
  ./configure \
  --prefix=${INSTALL_DIR}

RUN make -j$(nproc) \
  && make install \
  && make clean \
  && rm -rf ${BUILD_DIR}/oniguruma/

# Build Libzip
# Needed by:
#   - php-zip

WORKDIR ${BUILD_DIR}/libzip/

RUN set -e \
  && LIBZIP_VERSION=1.10.1 \
  && curl --location --silent --show-error --fail https://github.com/nih-at/libzip/releases/download/v${LIBZIP_VERSION}/libzip-${LIBZIP_VERSION}.tar.gz \
  | tar xzC . --strip-components=1

RUN CFLAGS="-O3" \
  CPPFLAGS="-I${INSTALL_DIR}/include  -I/usr/include" \
  LDFLAGS="-L${INSTALL_DIR}/lib64 -L${INSTALL_DIR}/lib" \
  cmake \
  -DCMAKE_INSTALL_PREFIX=${INSTALL_DIR} \
  -DCMAKE_BUILD_TYPE=RELEASE

RUN make -j$(nproc) \
  && make install \
  && make clean \
  && rm -rf ${BUILD_DIR}/libzip/

# Build OpenSSL
# Needs:
#   - zlib
#   - perl
# Needed by:
#   - curl
#   - php-openssl

WORKDIR ${BUILD_DIR}/openssl/

RUN set -e \
  && OPENSSL_VERSION=3.2.1 \
  && curl --location --silent --show-error --fail https://github.com/openssl/openssl/releases/download/openssl-${OPENSSL_VERSION}/openssl-${OPENSSL_VERSION}.tar.gz \
  | tar xzC . --strip-components=1

RUN CFLAGS="-O3" \
  CPPFLAGS="-I${INSTALL_DIR}/include  -I/usr/include" \
  LDFLAGS="-L${INSTALL_DIR}/lib64 -L${INSTALL_DIR}/lib" \
  ./Configure \
  --prefix=${INSTALL_DIR} \
  --openssldir=${INSTALL_DIR}/sigan/ssl \
  --release \
  enable-tls1_3 \
  no-tests \
  shared \
  zlib

RUN make -j$(nproc) \
  && make install -p ${INSTALL_DIR}/sigan/ssl \
  && CA_BUNDLE_SOURCE="https://curl.se/ca/cacert.pem" CA_BUNDLE="${INSTALL_DIR}/sigan/ssl/cert.pem" && curl -Lk -o ${CA_BUNDLE} ${CA_BUNDLE_SOURCE} \
  && make clean \
  && rm -rf ${BUILD_DIR}/openssl/

# Build Libxml2
# Needs:
#   - zlib
# Needed by:
#   - php-curl
#   - libnghttp2

WORKDIR  ${BUILD_DIR}/xml2/

RUN set -e \
  && XML2_VERSION=2.12.5 \
  && curl --location --silent --show-error --fail https://download.gnome.org/sources/libxml2/${XML2_VERSION%.*}/libxml2-${XML2_VERSION}.tar.xz \
  | tar xJC . --strip-components=1

RUN CFLAGS="-O3" \
  CPPFLAGS="-I${INSTALL_DIR}/include -I/usr/include" \
  LDFLAGS="-L${INSTALL_DIR}/lib64 -L${INSTALL_DIR}/lib" \
  ./configure \
  --prefix=${INSTALL_DIR} \
  --with-sysroot=${INSTALL_DIR} \
  --enable-shared \
  --disable-static \
  --with-html \
  --with-history \
  --enable-ipv6=no \
  --with-icu \
  --with-zlib \
  --without-python

RUN make -j$(nproc) \
  && make install \
  && cp xml2-config ${INSTALL_DIR}/bin/xml2-config \
  && make clean \
  && rm -rf ${BUILD_DIR}/xml2/

# Build Libssh2.
# Needs:
#   - zlib
#   - OpenSSL
# Needed by:
#   - curl

WORKDIR  ${BUILD_DIR}/libssh2/

RUN set -e \
  && LIBSSH2_VERSION=1.11.0 \
  && curl --location --silent --show-error --fail https://github.com/libssh2/libssh2/releases/download/libssh2-${LIBSSH2_VERSION}/libssh2-${LIBSSH2_VERSION}.tar.gz \
  | tar xzC . --strip-components=1

RUN CFLAGS="-O3" \
  CPPFLAGS="-I${INSTALL_DIR}/include -I/usr/include" \
  LDFLAGS="-L${INSTALL_DIR}/lib64 -L${INSTALL_DIR}/lib" \
  ./configure \
  --prefix=${INSTALL_DIR} \
  --enable-shared \
  --with-openssl \
  --with-libz \
  --disable-debug \
  --disable-deprecated \
  --disable-examples-build \
  --disable-docker-tests \
  --disable-sshd-tests

RUN make -j$(nproc) \
  && make install \
  && make clean \
  && rm -rf ${BUILD_DIR}/libssh2/

# Build libnghttp2.
# Needs:
#   - zlib
#   - libxml2
# Needed by:
#   - curl

WORKDIR  ${BUILD_DIR}/nghttp2

RUN set -e \
  && NGHTTP2_VERSION=1.59.0 \
  && curl --location --silent --show-error --fail https://github.com/nghttp2/nghttp2/releases/download/v${NGHTTP2_VERSION}/nghttp2-${NGHTTP2_VERSION}.tar.gz \
  | tar xzC . --strip-components=1

RUN CFLAGS="-O3" \
  CPPFLAGS="-I${INSTALL_DIR}/include -I/usr/include" \
  LDFLAGS="-L${INSTALL_DIR}/lib64 -L${INSTALL_DIR}/lib" \
  ./configure \
  --prefix=${INSTALL_DIR} \
  --enable-lib-only \
  --enable-http3

RUN make -j$(nproc) \
  && make install \
  && make clean \
  && rm -rf ${BUILD_DIR}/nghttp2/

# Build libpsl
# Needed by:
#   - curl

WORKDIR ${BUILD_DIR}/libpsl/

RUN set -e \
  && LIBPSL_VERSION=0.21.5 \
  && curl --location --silent --show-error --fail https://github.com/rockdaboot/libpsl/releases/download/${LIBPSL_VERSION}/libpsl-${LIBPSL_VERSION}.tar.gz \
  | tar xzC . --strip-components=1

RUN CFLAGS="-O3" \
  CPPFLAGS="-I${INSTALL_DIR}/include -I/usr/include" \
  LDFLAGS="-L${INSTALL_DIR}/lib64 -L${INSTALL_DIR}/lib" \
  ./configure \
  --prefix=${INSTALL_DIR}

RUN make -j$(nproc) \
  && make install \
  && make clean \
  && rm -rf ${BUILD_DIR}/libpsl/

# Build curl
# Needs:
#   - zlib
#   - OpenSSL
#   - libssh2
#   - libnghttp2
#   - libpsl
# Needed by:
#   - php-curl

WORKDIR  ${BUILD_DIR}/curl/

RUN set -e \
  && CURL_VERSION=8.6.0 \
  && curl --location --silent --show-error --fail https://github.com/curl/curl/releases/download/curl-${CURL_VERSION//./_}/curl-${CURL_VERSION}.tar.gz \
  | tar xzC . --strip-components=1

RUN CFLAGS="-O3" \
  CPPFLAGS="-I${INSTALL_DIR}/include -I/usr/include" \
  LDFLAGS="-L${INSTALL_DIR}/lib64 -L${INSTALL_DIR}/lib" \
  ./configure \
  --prefix=${INSTALL_DIR} \
  --with-ca-bundle=${CA_BUNDLE} \
  --enable-optimize \
  --disable-warnings \
  --disable-dependency-tracking \
  --with-zlib \
  --enable-http \
  --enable-ftp  \
  --enable-file \
  --enable-proxy  \
  --enable-tftp \
  --enable-ipv6 \
  --enable-openssl-auto-load-config \
  --enable-cookies \
  --with-gnu-ld \
  --with-ssl \
  --with-libssh2 \
  --with-nghttp2

RUN make -j$(nproc) \
  && make install \
  && make clean \
  && rm -rf ${BUILD_DIR}/curl/

# Build ImageMagick
# Needs:
#   - zlib
#   - libxml2
# Needed by:
#   - php-imagick

WORKDIR ${BUILD_DIR}/imagemagick/

RUN set -e \
  && IMAGEMAGICK_VERSION=7.1.1-28 \
  && curl --location --silent --show-error --fail https://github.com/ImageMagick/ImageMagick/archive/refs/tags/${IMAGEMAGICK_VERSION}.tar.gz \
  | tar xzC . --strip-components=1

RUN CFLAGS="-O3" \
  CPPFLAGS="-I${INSTALL_DIR}/include -I/usr/include" \
  LDFLAGS="-L${INSTALL_DIR}/lib64 -L${INSTALL_DIR}/lib" \
  ./configure \
  --prefix=${INSTALL_DIR} \
  --enable-shared \
  --disable-static \
  --with-quantum-depth=16 \
  --disable-openmp \
  --without-threads \
  --without-perl \
  --without-magick-plus-plus \
  --without-x

RUN make -j$(nproc) \
  && make install \
  && make clean \
  && rm -rf ${BUILD_DIR}/imagemagick/

################################
### Build PHP and Extensions ###
################################

FROM build-dependencies as build-php

WORKDIR ${BUILD_DIR}/php/

RUN set -e \
  && PHP_VERSION=8.3.2 \
  && curl --location --silent --show-error --fail https://github.com/php/php-src/archive/refs/tags/php-${PHP_VERSION}.tar.gz \
  | tar xzC . --strip-components=1

RUN ./buildconf --force

# Installing only the required extensions by WP to reduce the size of the library.
# Source: https://make.wordpress.org/hosting/handbook/server-environment/#php-extensions
# Node: libxml is required by dom
RUN CFLAGS="-fstack-protector-strong -fpic -fpie -O3 -I${INSTALL_DIR}/include -I/usr/include -ffunction-sections -fdata-sections" \
  CPPFLAGS="-fstack-protector-strong -fpic -fpie -O3 -I${INSTALL_DIR}/include -I/usr/include -ffunction-sections -fdata-sections" \
  LDFLAGS="-L${INSTALL_DIR}/lib64 -L${INSTALL_DIR}/lib -Wl,-O1 -Wl,--hash-style=both -pie" \
  ./configure \
  --prefix=${INSTALL_DIR} \
  --enable-option-checking=fatal \
  --with-config-file-path=${INSTALL_DIR}/sigan/config \
  # Disable extensions and binaries that are not needed
  --disable-all \
  # --disable-cli \
  --disable-phpdbg \
  --disable-cgi \
  # Enable desired binaries
  --enable-embed \
  # Required by WP
  --with-mysqli \
  # Highly Recommended by WP
  --with-curl \
  --enable-dom \
  --with-libxml \
  --enable-exif \
  --enable-fileinfo \
  --enable-intl \
  --enable-mbstring \
  --with-openssl \
  --enable-xml \
  --with-zip \
  # Others
  --enable-session \
  --enable-bcmath \
  --enable-filter \
  --enable-shmop \
  --enable-opcache \
  --with-zlib \
  --with-gettext

RUN make -j$(nproc) \
  && make install \
  && make clean

# Build igbinary extension

WORKDIR ${BUILD_DIR}/igbinary/

RUN set -e \
  && IGBINARY_VERSION=3.2.15 \
  && curl --location --silent --show-error --fail https://github.com/igbinary/igbinary/archive/refs/tags/${IGBINARY_VERSION}.tar.gz \
  | tar xzC . --strip-components=1

RUN phpize
RUN CFLAGS="-O3" \
  ./configure \
  --enable-igbinary

RUN make -j$(nproc) \
  && make install \
  && make clean \
  && rm -rf ${BUILD_DIR}/igbinary/

# Build redis extension

WORKDIR ${BUILD_DIR}/phpredis/

RUN set -e \
  && PHPREDIS_VERSION=6.0.2 \
  && curl --location --silent --show-error --fail https://github.com/phpredis/phpredis/archive/refs/tags/${PHPREDIS_VERSION}.tar.gz \
  | tar xzC . --strip-components=1

RUN phpize
RUN CFLAGS="-O3" \
  ./configure \
  --enable-redis-igbinary

RUN make -j$(nproc) \
  && make install \
  && make clean \
  && rm -rf ${BUILD_DIR}/phpredis/

# Build imagick extension

WORKDIR ${BUILD_DIR}/imagick/

RUN set -e \
  && IMAGICK_VERSION=3.7.0 \
  && curl --location --silent --show-error --fail https://github.com/Imagick/imagick/archive/refs/tags/${IMAGICK_VERSION}.tar.gz \
  | tar xzC . --strip-components=1

RUN phpize
RUN CFLAGS="-O3" \
  ./configure

RUN make -j$(nproc) \
  && make install \
  && make clean \
  && rm -rf ${BUILD_DIR}/imagick/

WORKDIR ${INSTALL_DIR}

####################
### Install Rust ###
####################

FROM build-php as install-rust

WORKDIR ${BUILD_DIR}/rust/

# Install Rust

RUN set -e \
  && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable

ENV PATH=/root/.cargo/bin:${PATH}

# Install cargo-watch

RUN set -e \
  && CARGO_WATCH_VERSION=8.5.2 \  
  && curl --location --silent --show-error --fail https://github.com/watchexec/cargo-watch/releases/download/v${CARGO_WATCH_VERSION}/cargo-watch-v${CARGO_WATCH_VERSION}-aarch64-unknown-linux-gnu.tar.xz \
  | tar xJC /root/.cargo/bin --strip-components=1

# Install cargo-lambda

RUN set -e \
  && CARGO_LAMBDA_VERSION=1.2.1 \  
  && curl --location --silent --show-error --fail https://github.com/cargo-lambda/cargo-lambda/releases/download/v${CARGO_LAMBDA_VERSION}/cargo-lambda-v${CARGO_LAMBDA_VERSION}.aarch64-unknown-linux-musl.tar.gz \
  | tar -xzC /root/.cargo/bin

#####################################
### Setup Development Environment ###
#####################################

FROM public.ecr.aws/lambda/provided:al2023-arm64 as development

ENTRYPOINT []

ARG BUILD_DIR=/tmp/build
ARG INSTALL_DIR=/opt

# Copy PHP binaries

WORKDIR ${INSTALL_DIR}/bin/

COPY --from=build-php ${INSTALL_DIR}/bin/php .
COPY --from=build-php ${INSTALL_DIR}/bin/php-config .
# COPY --from=build-php ${INSTALL_DIR}/bin/php-cgi .
# COPY --from=build-php ${INSTALL_DIR}/sbin/php-fpm .

# Copy PHP extensions

WORKDIR ${INSTALL_DIR}/sigan/extensions/

COPY --from=build-php ${INSTALL_DIR}/lib/php/extensions/**/*.so .

# Copy PHP configuration

WORKDIR ${INSTALL_DIR}/sigan/config/

COPY ./config .

# Copy dependencies

WORKDIR ${INSTALL_DIR}/lib/

COPY --from=build-php ${INSTALL_DIR}/lib/libphp*.so .
COPY --from=build-php ${INSTALL_DIR}/lib/libicu*.so .
COPY --from=build-php ${INSTALL_DIR}/lib/libonig*.so .
COPY --from=build-php ${INSTALL_DIR}/lib/libssh2*.so .
COPY --from=build-php ${INSTALL_DIR}/lib/libpsl*.so .
COPY --from=build-php ${INSTALL_DIR}/lib64/libzip*.so .

# Copy PHP headers for Rust bindings

WORKDIR ${INSTALL_DIR}/include/php/

COPY --from=build-php ${INSTALL_DIR}/include/php .

# Install utilities needed for Rust bindings

RUN LD_LIBRARY_PATH= dnf install -y clang bzip2-devel

# Copy Rust and Cargo

COPY --from=install-rust /root/.cargo /root/.cargo
COPY --from=install-rust /root/.rustup /root/.rustup

ENV PATH=/root/.cargo/bin:${PATH}

# Set AWS Lambda environment variables

ENV AWS_LAMBDA_FUNCTION_NAME=runtime
ENV AWS_LAMBDA_FUNCTION_VERSION=1
ENV AWS_LAMBDA_FUNCTION_MEMORY_SIZE=2048
ENV AWS_LAMBDA_RUNTIME_API=http://127.0.0.1:8080/.rt

WORKDIR /mnt/runtime

######################
### Strip Binaries ###
######################

FROM build-php as strip-binaries

# RUN strip ${INSTALL_DIR}/bin/php-cgi
# RUN strip ${INSTALL_DIR}/sbin/php-fpm
RUN strip ${INSTALL_DIR}/lib/php/extensions/**/*.so
RUN strip ${INSTALL_DIR}/lib/libicu*.so
RUN strip ${INSTALL_DIR}/lib/libonig*.so
RUN strip ${INSTALL_DIR}/lib/libssh2*.so
RUN strip ${INSTALL_DIR}/lib/libpsl*.so
RUN strip ${INSTALL_DIR}/lib64/libzip*.so

####################################
### Setup Production Environment ###
####################################

FROM public.ecr.aws/lambda/provided:al2023-arm64 as production

ENTRYPOINT []

ARG INSTALL_DIR=/opt

# Copy PHP binaries

WORKDIR ${INSTALL_DIR}/bin/

# COPY --from=strip-binaries ${INSTALL_DIR}/bin/php-cgi .
# COPY --from=strip-binaries ${INSTALL_DIR}/sbin/php-fpm .

# Copy PHP extensions

WORKDIR ${INSTALL_DIR}/sigan/extensions/

COPY --from=strip-binaries ${INSTALL_DIR}/lib/php/extensions/**/*.so .

# Copy PHP configuration

WORKDIR ${INSTALL_DIR}/sigan/config/

COPY ./config/php-cgi.ini .
COPY ./config/php-fpm.ini .
COPY ./config/php-fpm.conf .

# Copy dependencies

WORKDIR ${INSTALL_DIR}/lib/

COPY --from=strip-binaries ${INSTALL_DIR}/lib/libphp*.so .
COPY --from=strip-binaries ${INSTALL_DIR}/lib/libicu*.so .
COPY --from=strip-binaries ${INSTALL_DIR}/lib/libonig*.so .
COPY --from=strip-binaries ${INSTALL_DIR}/lib/libssh2*.so .
COPY --from=strip-binaries ${INSTALL_DIR}/lib/libpsl*.so .
COPY --from=strip-binaries ${INSTALL_DIR}/lib64/libzip*.so .

WORKDIR /var/task