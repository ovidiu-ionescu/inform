FROM scratch
COPY target/x86_64-unknown-linux-musl/release/inform settings.toml ./
ENTRYPOINT ["./inform"]
EXPOSE 7878/tcp

