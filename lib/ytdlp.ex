defmodule Ytdlp do
  @vidtypes ["mp4", "webm"]

  def download(url) do
    {res, status_code} = System.cmd("yt-dlp", ["--get-url", url])
    case status_code do
      0 -> dl_src_url(res |> String.trim)
      _ -> {:error, res}
    end
  end

  def dl_src_url(url) do
    IO.inspect("Downloading #{url}")
    case HTTPoison.get(url) do
      {:ok, %HTTPoison.Response{status_code: 200, body: body}} ->
        ext = guess_type(url)
        {:ok, %{name: "video.#{ext}", body: body}}
      {:ok, %HTTPoison.Response{status_code: code}} -> {:error, "HTTP error: #{code}"}
      {:error, err} -> {:error, err}
    end
  end

  def guess_type(url) do
    Enum.find(@vidtypes, & String.contains?(url, &1))
  end
end
