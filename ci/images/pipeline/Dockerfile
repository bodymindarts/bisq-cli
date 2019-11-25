FROM rust

RUN rustup component add clippy

RUN git clone https://github.com/bats-core/bats-core.git \
            && cd bats-core \
            && ./install.sh /usr/local

RUN apt-get update \
            && apt-get install -y \
            jq vim

RUN wget https://download.java.net/java/GA/jdk10/10.0.2/19aef61b38124481863b1413dce1855f/13/openjdk-10.0.2_linux-x64_bin.tar.gz \
            && mkdir /java_home \
            && tar -C /java_home -xvf openjdk-10.0.2_linux-x64_bin.tar.gz \
            && rm openjdk-10.0.2_linux-x64_bin.tar.gz

ENV JAVA_HOME=/java_home/jdk-10.0.2
ENV PATH ${PATH}:/java_home/bin

RUN java -version
