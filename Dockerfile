FROM rust:1.84

# Install dependencies for building libvips
RUN apt-get update && apt-get install -y \
    libfftw3-dev \
    libopenexr-dev \
    libgsf-1-dev \
    libglib2.0-dev \
    liborc-dev \
    libopenslide-dev \
    libmatio-dev \
    libwebp-dev \
    libjpeg-dev \
    libexpat1-dev \
    libexif-dev \
    libtiff5-dev \
    libcfitsio-dev \
    libpoppler-glib-dev \
    librsvg2-dev \
    libpango1.0-dev \
    libopenjp2-7-dev \
    liblcms2-dev \
    libimagequant-dev \
    pkg-config \
    meson \
    ninja-build \
    wget \
    libssl-dev && rm -rf /var/lib/apt/lists/*

# Build libvips
WORKDIR "/vips-build"
RUN wget https://github.com/libvips/libvips/releases/download/v8.15.3/vips-8.15.3.tar.xz
RUN tar -xvJf vips-8.15.3.tar.xz
WORKDIR "/vips-build/vips-8.15.3"

RUN meson setup build --prefix=/usr/local
RUN ninja -C build
RUN ninja -C build install
RUN cd /usr/lib/ && ln -s /usr/local/lib/libvips.so.42 libvips.so.42
RUN ldconfig

# Build Rust application
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

EXPOSE 3000
CMD ["/app/target/release/iamge-vips"]