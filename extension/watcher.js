const port = browser.runtime.connect()

function sendSongInfo() {
  const song = document.querySelector(".title.style-scope.ytmusic-player-bar").innerText
  const artist = document.querySelector(".byline.style-scope.ytmusic-player-bar").innerText

  if (song && artist) {
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
