import express from 'express'
import cors from 'cors'
import DiscordRPC from 'discord-rpc'

const app = express()
const port = 2945
const clientId = '1300015589540106240'

const client = new DiscordRPC.Client({ transport: 'ipc' })

client.on('ready', () => {
  console.log(`connected to discord client`)
})

client.login({ clientId }).catch(console.error);

app.use(cors())
app.use(express.json())

app.post('/', (req, res) => {
  const { song, artist, art } = req.body
  setActivity(song, artist, art)
  res.send(req.body)
})

app.listen(port, () => {
  console.log(`listening on port ${port}`)
})

function setActivity(song, artist) {
  if (!client) {
    return;
  }

  client.setActivity({
    details: song,
    state: artist,
    instance: false,
  });
}
