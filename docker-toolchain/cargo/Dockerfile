FROM ubuntu:16.04
LABEL maintainer='Nathaniel van Diepen'

# Need to specify UID and GID so they match the external user.
# UNAME has no significance.
ARG UNAME=builder
ARG UID=1000
ARG GID=1000
ARG ostype=Linux

RUN apt-get -qq update
RUN apt-get -qq install curl build-essential gcc-arm-linux-gnueabihf vim

RUN groupadd -r --gid ${GID} ${UNAME}
RUN useradd -u $UID -g $GID -m $UNAME
USER $UNAME

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y
ENV PATH="${PATH}:/home/$UNAME/.cargo/bin"

RUN rustup target add armv7-unknown-linux-gnueabihf

# make the registry folder to ensure correct permissions
RUN mkdir -p "/home/$UNAME/.cargo/registry"

COPY .cargo/config /home/$UNAME/.cargo/config

USER root
RUN curl https://remarkable.engineering/deploy/sdk/poky-glibc-x86_64-meta-toolchain-qt5-cortexa9hf-neon-toolchain-2.1.3.sh \
  -o install-toolchain.sh
RUN chmod +x install-toolchain.sh
RUN ./install-toolchain.sh -y \
  -d /opt/poky/2.1.3/
USER $UNAME

RUN rustup update
