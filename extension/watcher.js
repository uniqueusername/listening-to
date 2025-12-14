const port = browser.runtime.connect()

function sendSongInfo() {
  const song = document.querySelector(".title.style-scope.ytmusic-player-bar")?.innerText
  const artist =
    document.querySelector("yt-formatted-string.byline.style-scope.ytmusic-player-bar .yt-simple-endpoint.style-scope.yt-formatted-string")?.innerText
      ?? document.querySelector("yt-formatted-string.byline.style-scope.ytmusic-player-bar .style-scope.yt-formatted-string")?.innerText
      ?? ""

  if (song) {
    port.postMessage({ song, artist })
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
