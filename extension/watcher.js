function sendSongInfo() {
  const song = document.querySelector(".title.style-scope.ytmusic-player-bar")?.innerText
  const artist =
    document.querySelector("yt-formatted-string.byline.style-scope.ytmusic-player-bar .yt-simple-endpoint.style-scope.yt-formatted-string")?.innerText
      ?? document.querySelector("yt-formatted-string.byline.style-scope.ytmusic-player-bar .style-scope.yt-formatted-string")?.innerText
      ?? ""
  const bigImage = document.querySelector("#song-image img")?.src
  const art = (bigImage && !bigImage.includes("data:image.gif")) ? bigImage
    : document.querySelector(".image.style-scope.ytmusic-player-bar")?.src

  if (song) browser.runtime.sendMessage({ song, artist, art }).catch(() => {})
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
