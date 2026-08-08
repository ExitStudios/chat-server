const button = document.getElementById("submit-btn");
button?.addEventListener("click", async () => {
    postMessage({
        user: "Bernd",
        text: "Hello",
    });
    loadMessages();
});
async function loadMessages() {
    const response = await fetch("/api/messages");
    if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
    }
    const messages = await response.json();
    renderMessages(messages);
    return messages;
}
async function postMessage(message) {
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
function renderMessages(messages) {
    console.log(messages);
}
loadMessages();
export {};
//# sourceMappingURL=home.js.map