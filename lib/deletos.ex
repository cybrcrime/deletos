defmodule Deletos.Bot do
  use Nostrum.Consumer

  alias Nostrum.Api
  alias Nostrum.Struct.Embed

  @harmya_id 730844301608615946

  def handle_event({:READY, ready, _}) do
    self_id = ready.user.id
    {:ok, _} = Agent.start_link(fn -> self_id end, name: :self_id)
  end

  def handle_event({:MESSAGE_CREATE, msg, _}) do
    self_id = Agent.get(:self_id, &(&1))
    mentions_self = Enum.any?(msg.mentions, &(&1.id == self_id))

    if mentions_self do
      # could send multiple attachments in one message
      for {res, i} <- msg |> scrape_msg |> Enum.with_index do
        case res do
          {:ok, attachment} -> Api.create_message(msg.channel_id, file: attachment)
          {:error, err} -> 
            IO.inspect(err)
            Api.create_message(msg.channel_id, "Not ok (embed #{i})")
        end
      end
    end
  end

  def handle_event({:GUILD_MEMBER_UPDATE, {guild_id, old_user, new_user}, _}) do
    if old_user.user_id == @harmya_id && new_user.nick != "racist" do
      Api.modify_guild_member(guild_id, @harmya_id, nick: "racist", reason: "racist")
      |> IO.inspect()
    end
  end

  def handle_event(_), do: :ok

  def scrape_msg(msg) do
    for %Embed{url: url} <- msg.embeds, do: scrape(url)
  end

  def scrape(url, tries_left \\ 3) do
    case Ytdlp.download(url) do
      {:ok, attachment} -> {:ok, attachment}
      {:error, _} when tries_left > 0 -> scrape(url, tries_left - 1)
      {:error, err} -> {:error, err} # tries left == 0
    end
  end
end

defmodule Deletos.Application do
  use Application

  def start(_type, _args) do
    children = [Deletos.Bot]
    Supervisor.start_link(children, strategy: :one_for_one)
  end
end
