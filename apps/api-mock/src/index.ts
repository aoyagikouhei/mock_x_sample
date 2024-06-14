import { Hono } from 'hono'

const app = new Hono()

app.get('/2/users/me', (c) => {
  return c.json({data: {id: '123', name: 'abc', username: 'xyz'}})
})

export default app
