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

echo VK_API_APP_ID_1 > config/private_vk_api_app_ids
echo VK_API_APP_ID_2 >> config/private_vk_api_app_ids
echo VK_API_APP_ID_3 >> config/private_vk_api_app_ids

echo VK_API_TOKEN_1 > config/private_vk_api_tokens
echo VK_API_TOKEN_2 >> config/private_vk_api_tokens
echo VK_API_TOKEN_3 >> config/private_vk_api_tokens

TODO
touch config/private_patterns
cargo run
```

## Credits
Author: Mike Zueff

Email: mike.zueff [at] gmail.com
