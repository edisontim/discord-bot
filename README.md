# Discord bot for Eternum

## Installation

`cargo install cargo-shuttle@0.53.0`

## Running on a test environment

Make sure you have docker running

`shuttle run`

## Running on a prod environment

- Create an account on [shuttle.dev](https://www.shuttle.dev/)
- `shuttle login`
- Create a project `shuttle project create --name discord-bot` and get the id of the project `shuttle project list`
- For ease of use on subsequent shuttle commands, create a `.shuttle/config.toml` file a the root of your project and add: `id = "<your_project_id>"` in it
- When you want to deploy your project: `shuttle deploy`


## Adding an event

- Copy paste an event file such as [this one](./crates/discord-bot/src/events/season_ended.rs)
- Change your event as required, much sure the type definition matches the Cairo event. It needs to implement the `SubscriptionEvent` and `Metadata` trait
- Add it in the `TORII_SUBSCRIPTION_MODELS` array
- Add a match branch in the `treat_received_torii_event` to process it.