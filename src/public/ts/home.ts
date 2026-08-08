const button = document.getElementById("submit-btn");

button?.addEventListener("click", async () => {
  postMessage({
    user: "Bernd",
    text: "Hello",
  });

  loadMessages();
});

interface Message {
  user: string;
  text: string;
}

async function loadMessages(): Promise<Message[]> {
  const response = await fetch("/api/messages");

  if (!response.ok) {
    throw new Error(`HTTP ${response.status}`);
  }

  const messages: Message[] = await response.json();

  renderMessages(messages);

  return messages;
}

async function postMessage(message: Message) {
  const response = await fetch("/api/messages", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(message),
  });

  if (!response.ok) {
    throw new Error(`Failed to send message: HTTP ${response.status}`);
  }
}

function renderMessages(messages: Message[]): void {
  console.log(messages);
}

loadMessages();
