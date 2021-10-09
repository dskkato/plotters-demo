FROM rust:latest

WORKDIR /app
RUN apt update &&\
    rm -rf ~/.cache &&\
    apt clean all &&\
    apt install -y cmake &&\
    apt install -y clang
RUN apt install -y build-essential \
    libffi-dev libssl-dev zlib1g-dev \
    liblzma-dev libbz2-dev libreadline-dev \
    libsqlite3-dev libopencv-dev tk-dev libzmq3-dev \
    git


# install python
ENV HOME="/root"
ENV PYENV_ROOT="$HOME/.pyenv"
ENV PATH="${PYENV_ROOT}/shims:${PYENV_ROOT}/bin:${PATH}"
RUN git clone https://github.com/pyenv/pyenv.git $HOME/.pyenv
RUN echo 'eval "$(pyenv init -)"' >> ~/.bashrc
RUN eval "$(pyenv init -)"
RUN pyenv install 3.9.6
RUN pyenv global 3.9.6


# jupyterlab, evcxr
RUN pip install jupyterlab
RUN cargo install --git https://github.com/google/evcxr evcxr_jupyter --no-default-features
RUN evcxr_jupyter --install


ENTRYPOINT [ "/bin/bash" ]