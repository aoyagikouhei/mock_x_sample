import { Hono } from 'hono'

const app = new Hono()

app.get('/2/users/me', (c) => {
  return c.json({data: {id: '999', name: '予定表～①💖ﾊﾝｶｸだ', username: 'yoteihyo'}})
})

export default app
