const socket = new WebSocket("ws://localhost:3000/ws");
const messageDiv = document.getElementById("message");

socket.addEventListener("open", function (event) {
  socket.send("Hello Server!");
});

socket.addEventListener("message", function (event) {
  let messageP = document.createElement("p");
  messageP.textContent = "Message from server " + event.data;
  messageDiv.appendChild(messageP);
});

setTimeout(() => {
  const obj = { hello: "world" };
  const blob = new Blob([JSON.stringify(obj, null, 2)], {
    type: "application/json",
  });
  console.log("Sending blob over websocket");
  socket.send(blob);
}, 1000);

setTimeout(() => {
  socket.send("About done here...");
  console.log("Sending close over websocket");
  socket.close(3000, "Crash and Burn!");
}, 3000);
