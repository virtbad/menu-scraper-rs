FROM rust:alpine AS builder
WORKDIR /app

# Copy your Rust project to the container
COPY . .

# Install required dependencies for the binary to build
RUN apk add --no-cache yaml-dev musl-dev # openssl-dev

# Build the release binary
RUN cargo build --release

FROM alpine:latest as runner
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/menu-scraper-rs ./menu-scraper-rs

# Set the environment variables with default values
ENV WEBSITE ""
ENV API ""
ENV INITIAL_RUN "true"

# Create a cron job with redirection to log file
RUN echo "0 0 * * * /app/menu-scraper-rs" > /etc/crontabs/root

# Set permissions for cron
RUN chmod 600 /etc/crontabs/root

RUN echo '#!/bin/sh' > /app/initial.sh
# Run the binary on container start or skip if it's INITIAL_RUN is false
RUN echo 'if [ "$INITIAL_RUN" == "true" ]; then /app/menu-scraper-rs; else echo "Initial run skipped"; fi' >> /app/initial.sh
# Add forground cron to the end of the script
RUN echo "crond -f" >> /app/initial.sh
RUN chmod +x /app/initial.sh

CMD ["./initial.sh"]