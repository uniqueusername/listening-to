const wsAddress = "ws://localhost:9002"

let titleObjects = document.getElementsByClassName("title style-scope ytmusic-player-bar")
let artistObjects = document.getElementsByClassName("byline style-scope ytmusic-player-bar complex-string")

let title = titleObjects.length > 0 ? titleObjects[0].textContent : ""
let artist = artistObjects[0].firstElementChild.textContent

let ws = new WebSocket(wsAddress)
ws.onerror = console.log("websocket error")
ws.onopen = function () {
  console.log("websocket open")
  ws.send("goober goober")
}
