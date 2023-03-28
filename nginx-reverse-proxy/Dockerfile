FROM nginx:1.23.3 as reverse-proxy

COPY npm-expansions.conf /etc/nginx/nginx.conf

# We only need to compile rust build dependancies
# We only need static and pages directory
# We only need the build.rs file and can ignore created binaries