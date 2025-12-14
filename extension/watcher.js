const port = browser.runtime.connect()

function sendSongInfo() {
  const song = document.querySelector(".title.style-scope.ytmusic-player-bar")?.innerText
  const artist =
    document.querySelector("yt-formatted-string.byline.style-scope.ytmusic-player-bar .yt-simple-endpoint.style-scope.yt-formatted-string")?.innerText
      ?? document.querySelector("yt-formatted-string.byline.style-scope.ytmusic-player-bar .style-scope.yt-formatted-string")?.innerText
      ?? ""
  const art = document.querySelector(".image.style-scope.ytmusic-player-bar")?.src

  if (song) {
    port.postMessage({ song, artist, art })
  }
}

const observer = new MutationObserver(sendSongInfo)
const playerBar = document.querySelector("ytmusic-player-bar")

if (playerBar) {
  observer.observe(playerBar, {
    childList: true,
    subtree: true,
    characterData: true,
  })
}
