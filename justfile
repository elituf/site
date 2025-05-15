set windows-shell := ["pwsh", "-NoLogo", "-Command"]

server := "futile@futile.eu"
deploy := "/var/www/futile.eu"

deploy-site:
    scp -pr static/ index.css index.html index.js {{server}}:{{deploy}}
    ssh {{server}} 'chmod -R o+rX {{deploy}}/static/'

deploy-caddy:
    scp -pr Caddyfile {{server}}:Caddyfile
    ssh {{server}} 'sudo mv Caddyfile /etc/caddy/Caddyfile'
    ssh {{server}} 'sudo systemctl reload caddy'

deploy: deploy-site deploy-caddy

push-changes:
    git push -u origin main

full-deploy: push-changes deploy
