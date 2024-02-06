# syntax=docker/dockerfile:1

### Setup build environment ###

FROM public.ecr.aws/lambda/provided:al2023-arm64 as build

ARG BUILD_DIR=/tmp/build
ARG INSTALL_DIR=/opt

# Set PATH environment variable to include installed binaries
# Set LD_LIBRARY_PATH to include installed libraries
ARG PATH=${INSTALL_DIR}/bin:${PATH}
ARG LD_LIBRARY_PATH=${INSTALL_DIR}/lib:${INSTALL_DIR}/lib64:${LD_LIBRARY_PATH}

# Sets the number of parallel build jobs for CMake and Make
ARG CMAKE_BUILD_PARALLEL_LEVEL=4

### Install dependencies ###

RUN LD_LIBRARY_PATH= dnf install -y tar
RUN LD_LIBRARY_PATH= dnf install -y gzip
RUN LD_LIBRARY_PATH= dnf install -y gcc
RUN LD_LIBRARY_PATH= dnf install -y g++
RUN LD_LIBRARY_PATH= dnf install -y autoconf
RUN LD_LIBRARY_PATH= dnf install -y automake
RUN LD_LIBRARY_PATH= dnf install -y libtool
RUN LD_LIBRARY_PATH= dnf install -y openssl-devel
RUN LD_LIBRARY_PATH= dnf install -y libcurl-devel
RUN LD_LIBRARY_PATH= dnf install -y libxml2-devel
RUN LD_LIBRARY_PATH= dnf install -y libicu-devel
RUN LD_LIBRARY_PATH= dnf install -y oniguruma-devel
RUN LD_LIBRARY_PATH= dnf install -y libzip-devel
RUN LD_LIBRARY_PATH= dnf install -y ImageMagick-devel

### Build PHP ###

ARG PHP_VERSION=8.3.2

WORKDIR ${BUILD_DIR}/php/

RUN curl --location --silent --show-error --fail https://www.php.net/get/php-${PHP_VERSION}.tar.gz/from/this/mirror \
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
  --with-config-file-path=/opt/sigan/etc/php \
  # Disable extensions and binaries that are not needed
  --disable-all \
  --disable-cgi \
  --disable-cli \
  --disable-phpdbg \
  # Enable FPM as the only binary
  --enable-fpm \
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
  --enable-bcmath \
  --enable-filter \
  --enable-shmop \
  --enable-opcache \
  --enable-intl \
  --with-iconv \
  --with-zlib \
  --with-gettext

RUN make -j $(nproc)
RUN make install
RUN make clean

### Build igbinary extension ###

WORKDIR ${BUILD_DIR}

RUN git clone https://github.com/igbinary/igbinary.git

WORKDIR ${BUILD_DIR}/igbinary/

RUN phpize
RUN CFLAGS="-O3" \
    ./configure \
    --enable-igbinary
RUN make -j $(nproc)
RUN make install
RUN make clean

### Build redis extension ###

WORKDIR ${BUILD_DIR}

RUN git clone https://github.com/phpredis/phpredis.git

WORKDIR ${BUILD_DIR}/phpredis/

RUN phpize
RUN CFLAGS="-O3" \
    ./configure \
    --enable-redis-igbinary
RUN make -j $(nproc)
RUN make install
RUN make clean

### Build imagick extension ###

WORKDIR ${BUILD_DIR}

RUN git clone https://github.com/Imagick/imagick

WORKDIR ${BUILD_DIR}/imagick/

RUN phpize
RUN CFLAGS="-O3" \
    ./configure
RUN make -j $(nproc)
RUN make install
RUN make clean

CMD ["/bin/sh"]
