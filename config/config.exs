import Config

config :nostrum,
  token: File.read!("config/token.secret") |> String.trim(),
  gateway_intents: :all
