server := "futile@futile.eu"

deploy-wasm:
    trunk build --release
    rm -r dist/static/www
    ssh {{server}} 'rm -rf /var/www/futile.eu/dist/'
    scp -pr dist/ {{server}}:/var/www/futile.eu/dist/

deploy-caddy:
    scp -pr Caddyfile {{server}}:Caddyfile
    ssh {{server}} 'sudo mv Caddyfile /etc/caddy/Caddyfile'
    ssh {{server}} 'sudo systemctl reload caddy'

deploy: deploy-wasm deploy-caddy
