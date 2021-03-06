# social_media_sieve (SMS)
Big data analytics for social media written in Rust

## Tools and technologies
- Rust language
- Diesel
- SQLite
- Serde
- VK API

## Social media

### Currently supported social networks
- VK

### Roadmap
- Facebook
- LiveJournal

## Usage
```
git clone https://github.com/mike-zueff/social_media_sieve.git
cd social_media_sieve
cat database/init.sql | sqlite3 database/sms_db_sqlite

cat > config/private_vk_api_current_token.json <<EOF
{
  "access_token": "TOKEN",
  "email": "EMAIL",
  "expires_in": 0,
  "user_id": ID
}
EOF

touch config/private_patterns
cargo run
```

## Credits
Author: Mike Zueff

Email: mike.zueff [at] gmail.com
