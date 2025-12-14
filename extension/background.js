browser.runtime.onConnect.addListener(port => {
  port.onMessage.addListener(async message => {
    const response = await fetch("http://localhost:2945", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(message),
    })
  })
})
