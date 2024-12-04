server := "futile@futile.eu"

deploy-wasm:
    trunk build --release
    ssh {{server}} 'rm -rf /var/www/futile.eu/dist/'
    scp -pr dist/ {{server}}:/var/www/futile.eu/dist/

deploy-caddy:
    scp -pr Caddyfile {{server}}:/home/futile/Caddyfile
    ssh {{server}} 'sudo mv /home/futile/Caddyfile /etc/caddy/Caddyfile'
    ssh {{server}} 'sudo systemctl reload caddy'

deploy: deploy-wasm deploy-caddy
