loadMessages();

async function loadMessages() {
  let messages;

  fetch("/api/messages")
    .then((response) => response.json())
    .then((data) => {
      renderMessages(data);
    });

  return messages;
}

function renderMessages(messages) {
  for (let i = 0; i < messages.length; i++) {
    let element = document.createElement("p");
    element.innerHTML = messages[i].user + ": " + messages[i].text;

    document.body.appendChild(element);
  }
}

let submitBtn = document.getElementById("submit-btn");
let input = document.getElementById("msg-ipt");

submitBtn.onclick = () => {
  let msg = input.value;

  fetch("/api/messages", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      user: "Bernd",
      text: msg,
    }),
  });

  loadMessages();
};
