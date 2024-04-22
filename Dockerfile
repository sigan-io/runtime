# syntax=docker/dockerfile:1

### Setup Build Environment ###

FROM public.ecr.aws/lambda/provided:al2023-arm64 as setup-build

ARG BUILD_DIR=/tmp/build
ARG INSTALL_DIR=/opt

# Set PATH environment variable to include installed binaries
ARG PATH=${INSTALL_DIR}/bin:${INSTALL_DIR}/sbin:${PATH}

# Set LD_LIBRARY_PATH to include installed libraries
ARG LD_LIBRARY_PATH=${INSTALL_DIR}/lib:${INSTALL_DIR}/lib64:${LD_LIBRARY_PATH}

# Set the PKG_CONFIG_PATH to include libraries built from source
ARG PKG_CONFIG_PATH=${INSTALL_DIR}/lib64/pkgconfig:${INSTALL_DIR}/lib/pkgconfig

# Install utilities

RUN LD_LIBRARY_PATH= dnf install -y tar
RUN LD_LIBRARY_PATH= dnf install -y gzip
RUN LD_LIBRARY_PATH= dnf install -y xz
RUN LD_LIBRARY_PATH= dnf install -y gcc
RUN LD_LIBRARY_PATH= dnf install -y g++
RUN LD_LIBRARY_PATH= dnf install -y re2c
RUN LD_LIBRARY_PATH= dnf install -y bison
RUN LD_LIBRARY_PATH= dnf install -y cmake
RUN LD_LIBRARY_PATH= dnf install -y autoconf
RUN LD_LIBRARY_PATH= dnf install -y automake
RUN LD_LIBRARY_PATH= dnf install -y libtool
RUN LD_LIBRARY_PATH= dnf install -y binutils
RUN LD_LIBRARY_PATH= dnf install -y perl
RUN LD_LIBRARY_PATH= dnf install -y glibc-locale-source

# Locale settings

RUN localedef -i en_US -f UTF-8 en_US.UTF-8

ARG LANGUAGE=en_US.UTF-8
ARG LC_ALL=en_US.UTF-8
ARG LANG=en_US.UTF-8

### Build Libraries ###

FROM setup-build as build-libraries

# Build Zlib
# Needed by:
#   - openssl
#   - php-zlib
#   - libzip

ARG ZLIB_VERSION=1.3.1

WORKDIR ${BUILD_DIR}/zlib/

RUN curl --location --silent --show-error --fail https://github.com/madler/zlib/releases/download/v${ZLIB_VERSION}/zlib-${ZLIB_VERSION}.tar.gz \
  | tar xzC . --strip-components=1

RUN CFLAGS="-O3" \
  CPPFLAGS="-I${INSTALL_DIR}/include  -I/usr/include" \
  LDFLAGS="-L${INSTALL_DIR}/lib64 -L${INSTALL_DIR}/lib" \
  ./configure \
  --prefix=${INSTALL_DIR}
RUN make -j$(nproc)
RUN make install

# Build ICU
# Needed by:
#   - php-intl

ARG ICU_VERSION=74.2

WORKDIR ${BUILD_DIR}/icu/

RUN curl --location --silent --show-error --fail https://github.com/unicode-org/icu/releases/download/release-${ICU_VERSION//./-}/icu4c-${ICU_VERSION//./_}-src.tgz \
  | tar xzC . --strip-components=1

WORKDIR ${BUILD_DIR}/icu/source/

RUN CFLAGS="-O3" \
  CPPFLAGS="-I${INSTALL_DIR}/include  -I/usr/include" \
  LDFLAGS="-L${INSTALL_DIR}/lib64 -L${INSTALL_DIR}/lib" \
  ./configure \
  --prefix=${INSTALL_DIR}
RUN make -j$(nproc)
RUN make install

# Build Oniguruma
# Needed by:
#   - php-mbstring

ARG ONIGURUMA_VERSION=6.9.9

WORKDIR ${BUILD_DIR}/oniguruma/

RUN curl --location --silent --show-error --fail https://github.com/kkos/oniguruma/releases/download/v${ONIGURUMA_VERSION}/onig-${ONIGURUMA_VERSION}.tar.gz \
  | tar xzC . --strip-components=1

RUN CFLAGS="-O3" \
  CPPFLAGS="-I${INSTALL_DIR}/include  -I/usr/include" \
  LDFLAGS="-L${INSTALL_DIR}/lib64 -L${INSTALL_DIR}/lib" \
  ./configure \
  --prefix=${INSTALL_DIR}
RUN make -j$(nproc)
RUN make install

# Build Libzip
# Needed by:
#   - php-zip

ARG LIBZIP_VERSION=1.10.1

WORKDIR ${BUILD_DIR}/libzip/

RUN curl --location --silent --show-error --fail https://github.com/nih-at/libzip/releases/download/v${LIBZIP_VERSION}/libzip-${LIBZIP_VERSION}.tar.gz \
  | tar xzC . --strip-components=1

RUN CFLAGS="-O3" \
  CPPFLAGS="-I${INSTALL_DIR}/include  -I/usr/include" \
  LDFLAGS="-L${INSTALL_DIR}/lib64 -L${INSTALL_DIR}/lib" \
  cmake \
  -DCMAKE_INSTALL_PREFIX=${INSTALL_DIR} \
  -DCMAKE_BUILD_TYPE=RELEASE
RUN make -j$(nproc)
RUN make install

# Build OpenSSL
# Needs:
#   - zlib
#   - perl
# Needed by:
#   - curl
#   - php-openssl

ARG OPENSSL_VERSION=3.2.1

ARG CA_BUNDLE_SOURCE="https://curl.se/ca/cacert.pem"
ARG CA_BUNDLE="${INSTALL_DIR}/sigan/ssl/cert.pem"

WORKDIR ${BUILD_DIR}/openssl/

RUN curl --location --silent --show-error --fail https://github.com/openssl/openssl/releases/download/openssl-${OPENSSL_VERSION}/openssl-${OPENSSL_VERSION}.tar.gz \
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
RUN make -j$(nproc)
RUN make install -p ${INSTALL_DIR}/sigan/ssl
RUN curl -Lk -o ${CA_BUNDLE} ${CA_BUNDLE_SOURCE}

# Build Libxml2
# Needs:
#   - zlib
# Needed by:
#   - php-curl
#   - libnghttp2

ARG XML2_VERSION=2.12.5

WORKDIR  ${BUILD_DIR}/xml2/

RUN curl --location --silent --show-error --fail https://download.gnome.org/sources/libxml2/${XML2_VERSION%.*}/libxml2-${XML2_VERSION}.tar.xz \
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
RUN make -j$(nproc)
RUN make install
RUN cp xml2-config ${INSTALL_DIR}/bin/xml2-config

# Build Libssh2.
# Needs:
#   - zlib
#   - OpenSSL
# Needed by:
#   - curl

ARG LIBSSH2_VERSION=1.11.0

WORKDIR  ${BUILD_DIR}/libssh2/

RUN curl --location --silent --show-error --fail https://github.com/libssh2/libssh2/releases/download/libssh2-${LIBSSH2_VERSION}/libssh2-${LIBSSH2_VERSION}.tar.gz \
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
RUN make -j$(nproc)
RUN make install

# Build libnghttp2.
# Needs:
#   - zlib
#   - libxml2
# Needed by:
#   - curl

ARG NGHTTP2_VERSION=1.59.0

WORKDIR  ${BUILD_DIR}/nghttp2

RUN curl --location --silent --show-error --fail https://github.com/nghttp2/nghttp2/releases/download/v${NGHTTP2_VERSION}/nghttp2-${NGHTTP2_VERSION}.tar.gz \
  | tar xzC . --strip-components=1

RUN CFLAGS="-O3" \
  CPPFLAGS="-I${INSTALL_DIR}/include -I/usr/include" \
  LDFLAGS="-L${INSTALL_DIR}/lib64 -L${INSTALL_DIR}/lib" \
  ./configure \
  --prefix=${INSTALL_DIR} \
  --enable-lib-only \
  --enable-http3
RUN make -j$(nproc)
RUN make install

# Build libpsl
# Needed by:
#   - curl

ARG LIBPSL_VERSION=0.21.5

WORKDIR ${BUILD_DIR}/libpsl/

RUN curl --location --silent --show-error --fail https://github.com/rockdaboot/libpsl/releases/download/${LIBPSL_VERSION}/libpsl-${LIBPSL_VERSION}.tar.gz \
  | tar xzC . --strip-components=1

RUN CFLAGS="-O3" \
  CPPFLAGS="-I${INSTALL_DIR}/include -I/usr/include" \
  LDFLAGS="-L${INSTALL_DIR}/lib64 -L${INSTALL_DIR}/lib" \
  ./configure \
  --prefix=${INSTALL_DIR}
RUN make -j$(nproc)
RUN make install

# Build curl
# Needs:
#   - zlib
#   - OpenSSL
#   - libssh2
#   - libnghttp2
#   - libpsl
# Needed by:
#   - php-curl

ARG CURL_VERSION=8.6.0

WORKDIR  ${BUILD_DIR}/curl/

RUN curl --location --silent --show-error --fail https://github.com/curl/curl/releases/download/curl-${CURL_VERSION//./_}/curl-${CURL_VERSION}.tar.gz \
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
RUN make -j$(nproc)
RUN make install

# Build ImageMagick
# Needs:
#   - zlib
#   - libxml2
# Needed by:
#   - php-imagick

ARG IMAGEMAGICK_VERSION=7.1.1-28

WORKDIR ${BUILD_DIR}/imagemagick/

RUN curl --location --silent --show-error --fail https://github.com/ImageMagick/ImageMagick/archive/refs/tags/${IMAGEMAGICK_VERSION}.tar.gz \
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
RUN make -j$(nproc)
RUN make install

### Build PHP ###

FROM build-libraries as build-php

ARG PHP_VERSION=8.3.2

WORKDIR ${BUILD_DIR}/php/

RUN curl --location --silent --show-error --fail https://github.com/php/php-src/archive/refs/tags/php-${PHP_VERSION}.tar.gz \
  | tar xzC . --strip-components=1

ARG STRIP=--strip-debug

RUN ./buildconf --force

# Installing only the required extensions by WP to reduce the size of the library.
# Source: https://make.wordpress.org/hosting/handbook/server-environment/#php-extensions
# Node: libxml is required by dom
RUN CFLAGS="-fstack-protector-strong -fpic -fpie -O3 -I${INSTALL_DIR}/include -I/usr/include -ffunction-sections -fdata-sections" \
  CPPFLAGS="-fstack-protector-strong -fpic -fpie -O3 -I${INSTALL_DIR}/include -I/usr/include -ffunction-sections -fdata-sections" \
  LDFLAGS="-L${INSTALL_DIR}/lib64 -L${INSTALL_DIR}/lib -Wl,-O1 -Wl,${STRIP} -Wl,--hash-style=both -pie" \
  ./configure \
  --prefix=${INSTALL_DIR} \
  --enable-option-checking=fatal \
  --with-config-file-path=${INSTALL_DIR}/sigan/config \
  # Disable extensions and binaries that are not needed
  --disable-all \
  --disable-cli \
  --disable-phpdbg \
  # Enable desired binaries
  --enable-fpm \
  --enable-cgi \
  --enable-embed \
  --enable-litespeed \
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
RUN make -j$(nproc)
RUN make install

# Build igbinary extension

ARG IGBINARY_VERSION=3.2.15

WORKDIR ${BUILD_DIR}/igbinary/

RUN curl --location --silent --show-error --fail https://github.com/igbinary/igbinary/archive/refs/tags/${IGBINARY_VERSION}.tar.gz \
  | tar xzC . --strip-components=1

RUN phpize
RUN CFLAGS="-O3" \
  ./configure \
  --enable-igbinary
RUN make -j$(nproc)
RUN make install

# Build redis extension

ARG PHPREDIS_VERSION=6.0.2

WORKDIR ${BUILD_DIR}/phpredis/

RUN curl --location --silent --show-error --fail https://github.com/phpredis/phpredis/archive/refs/tags/${PHPREDIS_VERSION}.tar.gz \
  | tar xzC . --strip-components=1

RUN phpize
RUN CFLAGS="-O3" \
  ./configure \
  --enable-redis-igbinary
RUN make -j$(nproc)
RUN make install

# Build imagick extension

ARG IMAGICK_VERSION=3.7.0

WORKDIR ${BUILD_DIR}/imagick/

RUN curl --location --silent --show-error --fail https://github.com/Imagick/imagick/archive/refs/tags/${IMAGICK_VERSION}.tar.gz \
  | tar xzC . --strip-components=1

RUN phpize
RUN CFLAGS="-O3" \
  ./configure
RUN make -j$(nproc)
RUN make install

# Strip binaries and libraries

RUN strip ${INSTALL_DIR}/bin/php-cgi
RUN strip ${INSTALL_DIR}/sbin/php-fpm
RUN strip ${INSTALL_DIR}/lib/php/extensions/*/*
RUN strip ${INSTALL_DIR}/lib/libicu*.so*
RUN strip ${INSTALL_DIR}/lib/libonig*.so*
RUN strip ${INSTALL_DIR}/lib/libssh2*.so*
RUN strip ${INSTALL_DIR}/lib/libpsl*.so*
RUN strip ${INSTALL_DIR}/lib64/libzip*.so*

### Build PHP Bindings ###

FROM build-php as build-bindings

# Install Rust

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable

ENV PATH=/root/.cargo/bin:${PATH}

# Install dependencies

RUN LD_LIBRARY_PATH= dnf install -y clang
RUN LD_LIBRARY_PATH= dnf install -y bzip2-devel

# Generate bindings

WORKDIR /mnt/runtime/php-sys/

ENTRYPOINT [ "cargo" ]

### Prepare PHP Layer ###

FROM public.ecr.aws/lambda/provided:al2023-arm64 as php

ARG INSTALL_DIR=/opt

# Copy PHP binaries

WORKDIR ${INSTALL_DIR}/bin/

COPY --from=build-php ${INSTALL_DIR}/bin/php-cgi .
COPY --from=build-php ${INSTALL_DIR}/sbin/php-fpm .

# Copy PHP extensions

WORKDIR ${INSTALL_DIR}/sigan/extensions/

COPY --from=build-php ${INSTALL_DIR}/lib/php/extensions/*/* .

# Copy PHP configuration

WORKDIR ${INSTALL_DIR}/sigan/config/

COPY ./config/php-cgi.ini .
COPY ./config/php-fpm.ini .
COPY ./config/php-fpm.conf .

# Copy dependencies

WORKDIR ${INSTALL_DIR}/lib/

COPY --from=build-php ${INSTALL_DIR}/lib/libphp*.so* .
COPY --from=build-php ${INSTALL_DIR}/lib/libicu*.so* .
COPY --from=build-php ${INSTALL_DIR}/lib/libonig*.so* .
COPY --from=build-php ${INSTALL_DIR}/lib/libssh2*.so* .
COPY --from=build-php ${INSTALL_DIR}/lib/libpsl*.so* .
COPY --from=build-php ${INSTALL_DIR}/lib64/libzip*.so* .

WORKDIR /var/task

### Runtime Development ###

FROM build-php as runtime-dev

ENV RUNTIME_DEV_DIR=/mnt/runtime
ENV RUNTIME_DIR=/var/task
ENV WORDPRESS_DIR=/mnt/wordpress

WORKDIR ${RUNTIME_DEV_DIR}

# Install Rust

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable

ENV PATH=/root/.cargo/bin:${PATH}

RUN cargo install cargo-watch
RUN cargo install cargo-lambda

# Install utilities

RUN dnf install -y lsof
RUN dnf install -y socat

# Reduce image size

RUN rm /lambda-entrypoint.sh
RUN rm /usr/local/bin/aws-lambda-rie
RUN rm -rf /root/.cargo/registry
RUN rm -rf /usr/lib64/python3.9
RUN rm -rf /usr/lib/python3.9
RUN rm -rf /usr/lib64/perl5
RUN rm -rf /usr/share/perl5
RUN rm -rf /usr/share/cmake
RUN rm -rf /usr/bin/ctest
RUN rm -rf /usr/bin/cmake

# Set AWS Lambda environment variables

ENV AWS_LAMBDA_FUNCTION_NAME=runtime
ENV AWS_LAMBDA_FUNCTION_VERSION=1
ENV AWS_LAMBDA_FUNCTION_MEMORY_SIZE=2048
ENV AWS_LAMBDA_RUNTIME_API=http://127.0.0.1:8080/.rt

# Setup entrypoint

COPY ./entrypoint.sh /entrypoint.sh

ENTRYPOINT [ "/entrypoint.sh" ]