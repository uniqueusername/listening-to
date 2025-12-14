import express from 'express'
import cors from 'cors'

const app = express()
const port = 2945

app.use(cors())
app.use(express.json())

app.post('/', (req, res) => {
  res.send(req.body)
})

app.listen(port, () => {
  console.log(`listening on port ${port}`)
})
