FROM node:14-slim

RUN apt-get update && apt-get install -yq git python build-essential curl

# install rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH=/root/.cargo/bin:$PATH
RUN echo $PATH

# install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# create lib directory
RUN mkdir /lib
WORKDIR /lib

# copy sources
COPY . /lib

# install dependencies
RUN npm ci

RUN chmod +x ./npm-ci-publish-beta-only.sh
RUN chmod +x ./npm-ci-publish.sh

# set to production
RUN export NODE_ENV=production

# build
RUN npm run build:prod

CMD ["npm", "run", "test"]
