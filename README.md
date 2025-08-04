# actix-web-lb-connection
Reproducible example for showing client timeouts when Actix can't parse body

# To reproduce
docker network create web
docker compose up -d

Run this a couple of times and curl will eventually timeout (typically on my machine, every other time)
curl -H"Host: simple.localhost" http://localhost:8000 -d '{}' -m 5