# social_media_sieve (SMS)
Big data analytics for social media written in Rust

## Tools and technologies
- Rust language
- Serde
- Diesel
- SQLite
- VK API
- TODO

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
touch config/private_vk_tokens
touch config/private_patterns
cargo run
```
- TODO
