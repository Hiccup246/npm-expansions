# Questions
# - What do we do about our static directory and reasources which need to be available to
#   our runner build stage
# - Why have a folder directory of /usr/src/app in the runner build stage

# Creates a new build stage using rust:1.67.1 as the base image and names it to builder
FROM rust:1.67.1 as builder

# Default user is root but we set it to be explicit and remind us that the user is root
USER root

# Create a new rust binary package called rust-docker-web
RUN cargo new --bin rust-docker-web

# Set working directory to root of new rust project
WORKDIR /rust-docker-web

# Copy Projects Cargo.toml file to a new Cargo.toml file in filesystem of the container
COPY ./Cargo.toml ./Cargo.toml

# Generate a cargo release build
RUN cargo build --release

# Remove default cargo new project code
RUN rm src/*.rs

# Copy all project files into the filesystem of the container
ADD . ./

# Remove binary built from the inital cargo build --release command
RUN rm ./target/release/deps/rust_docker_web*

# Re build the project for release using all the new project files
RUN cargo build --release

FROM debian:bullseye-slim as runner
ARG APP=/usr/src/app

# Install dependancies (guide has tzdata and ca-certificates)
# RUN apt-get update \
#    && apt-get install -y ca-certificates tzdata \
#    This command removes the lists which are used to figure out which packages are available to install
#    && rm -rf /var/lib/apt/lists/*

# Tells docker that the container listens on port 8000 using TCP by defualt (my web server should listen to this port)
EXPOSE 8000

# Set TimeZone
# ENV TZ=Etc/UTC \
#    APP_USER=appuser

# Set new user based on idea of least privledges (we do not need root permissions)
ENV APP_GROUP=appgroup \
    APP_USER=dave

# Create a Unix group called appuser
# Add a new user called dave and add them to Unix group appgroup using -g
# Make a new directory and add parent directories if required. In this case /usr/src/app is created
RUN groupadd $APP_USER \
    && useradd -g $APP_GROUP $APP_USER \
    && mkdir -p ${APP}

# Copy binary executable from builder step into new directory namely /usr/src/app/rust-docker-web
COPY --from=builder /rust-docker-web/target/release/rust-docker-web ${APP}/rust-docker-web

# Change ownership of /usr/src/app to our dave user in the appgroup Unix group
RUN chown -R $APP_USER:$APP_USER ${APP}

# Sets the username to dace
USER $APP_USER

# Sets the working directory to /usr/src/app
WORKDIR ${APP}

# Execute our web application binary
CMD ["./rust-docker-web"]