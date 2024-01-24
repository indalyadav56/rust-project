# Base image
FROM rust:1.57 as builder

# Create appuser
ENV USER=appuser
ENV UID=10001 

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

# Set work directory  
WORKDIR /app

# Only copy manifests and src  
COPY Cargo.* ./
COPY src ./src

# Build release  
RUN cargo build --release 

# Runtime image
FROM debian:bullseye-slim

# Copy user from builder 
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

# Copy release from target 
COPY --from=builder --chown=${UID}:0 /app/target/release /app  

# Use appuser 
USER ${UID}:0  

# Run app
CMD ["/app/server"]