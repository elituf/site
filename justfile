server := "futile@futile.eu"
deploy := "/var/www/futile.eu"

deploy-site:
    scp -pr index.html static/ {{server}}:{{deploy}}

deploy-caddy:
    scp -pr Caddyfile {{server}}:Caddyfile
    ssh {{server}} 'sudo mv Caddyfile /etc/caddy/Caddyfile'
    ssh {{server}} 'sudo systemctl reload caddy'

deploy: deploy-site deploy-caddy
