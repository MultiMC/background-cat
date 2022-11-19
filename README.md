# Background Cat
## A Discord bot to parse MultiMC Logs and warn users about common errors

I made this bot because it was brought to my attention that other discords also had similar bots. Dissatisfied with the other ones, I made yet another one.

[![xkcd "Standards" comic](https://imgs.xkcd.com/comics/standards.png)](https://xkcd.com/927/)

## Development

- Build the code: `cargo build --bin discord-cat`
- Copy `.env.example` to `target/debug/`
- Customize `target/debug/.env` and include your Discord and paste.ee tokens in it, like this:
   ```
   # See https://docs.rs/env_logger
   RUST_LOG=error

   DISCORD_TOKEN=AAa0AAa0AAAaAaa...
   PASTE_EE_TOKEN=aaAAaAa0AaAA0...
   BACKGROUND_CAT_PREFIX=-
   ```
- Run the bot: `cargo run -p discord-cat`

## Running in production

- Copy `.env.example` to `.env`
- Customize `.env` and include your Discord and paste.ee tokens in it, like this:
   ```
   # See https://docs.rs/env_logger
   RUST_LOG=error

   DISCORD_TOKEN=AAa0AAa0AAAaAaa...
   PASTE_EE_TOKEN=aaAAaAa0AaAA0...
   BACKGROUND_CAT_PREFIX=-
   ```

- Build the images: `docker-compose build`
- Start the service: `docker-compose start`
