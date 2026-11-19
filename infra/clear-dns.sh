#!/usr/bin/env bash
# A tunnel CNAME cannot coexist with other records of the same name. Before the
# edge stack creates its CNAMEs, remove A/AAAA records and any CNAME that does
# not point at a tunnel, at the apex and www. Idempotent: the stack's own
# *.cfargotunnel.com CNAMEs and all other record types (MX, TXT) stay.
# Usage: clear-dns.sh <domain>   (needs CLOUDFLARE_API_TOKEN)
set -euo pipefail
domain=${1:?domain}
api() { curl -fsS -H "Authorization: Bearer ${CLOUDFLARE_API_TOKEN:?}" "$@"; }
base=https://api.cloudflare.com/client/v4
zone=$(api "$base/zones?name=$domain" | jq -er '.result[0].id')
for name in "$domain" "www.$domain"; do
  api "$base/zones/$zone/dns_records?name=$name&per_page=100" |
    jq -r '.result[]
      | select(.type == "A" or .type == "AAAA"
               or (.type == "CNAME" and (.content | endswith(".cfargotunnel.com") | not)))
      | "\(.id) \(.type) \(.name)"' |
    while read -r id type rname; do
      api -X DELETE "$base/zones/$zone/dns_records/$id" >/dev/null
      echo "removed $type $rname"
    done
done
