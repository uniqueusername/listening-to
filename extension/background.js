browser.runtime.onMessage.addListener(async message => {
  await fetch("http://localhost:2945", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(message),
  }).catch(err => console.error("failed to send update:", err))
})
